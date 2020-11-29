use dir::CopyOptions;
use serde::{Serialize, Deserialize};
use core::panic;
use std::io::prelude::*;
use std::env;
use std::fs;
use std::fmt;
use std::fs::File;

use reqwest;

extern crate flate2;
extern crate tar;

use flate2::read::GzDecoder;
use tar::Archive;

extern crate fs_extra;
use fs_extra::dir;

use std::process::Command;


const OSCONFIG_FILE_NAME: &str = "./.revos.json";
const HELP_STRING: &str = "Usage:\n\trock [init,create,edit] [args]";

trait Config<T> {
    fn empty() -> T;
    fn save(self) -> std::io::Result<()>;
    fn from_file(filepath: String) -> std::io::Result<T>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AVRPlatformSpecificConfig
{
    f_cpu: i32,
    mcu: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
enum PlatformSpecific {
    AVR(AVRPlatformSpecificConfig)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct OSConfig {
	drivers: Vec<String>,
    apps: Vec<String>,
    platform_spec: PlatformSpecific
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AppConfig {
    name: String,
    provides: String,
    requiers_apps: Vec<String>,
    requiers_drivers: Vec<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DriverConfig {
    name: String,
    provides: String,
    requiers: Vec<String>
}

impl AVRPlatformSpecificConfig {
    fn empty() -> AVRPlatformSpecificConfig {
        AVRPlatformSpecificConfig{f_cpu: 0, mcu: format!("")}
    }
}

impl Config<OSConfig> for OSConfig {
    fn empty() -> OSConfig {
        OSConfig{drivers: Vec::new(), apps: Vec::new(), platform_spec: PlatformSpecific::AVR(AVRPlatformSpecificConfig::empty())}
    }

    fn save(self) -> std::io::Result<()> {
        save_config(&self, OSCONFIG_FILE_NAME.to_owned())
    }

    fn from_file(filepath: String) -> std::io::Result<OSConfig> {
        Ok(serde_json::from_str(&fs::read_to_string(filepath).expect("Unable to read config"))?)
    }
}

impl fmt::Display for OSConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data: String = "[OSConfig]\n    Drivers:\n".to_owned();
        for i in &self.drivers {
            data = data + "      - " + i + "\n";
        }
        data = data + "    Apps:\n";
        for i in &self.apps {
            data = data + "      - " + i + "\n";
        }
        data = format!("{}{}", data, self.platform_spec);
        write!(f, "{}", data)
    }
}

impl Config<AppConfig> for AppConfig {
    fn empty() -> AppConfig {
        AppConfig{name: "".to_owned(), provides: "".to_owned(), requiers_apps: Vec::new(), requiers_drivers: Vec::new()}
    }

    fn save(self) -> std::io::Result<()> {
        save_config(&self, format!("./{}.app/.app.json", self.name.clone()))
    }

    fn from_file(filepath: String) -> std::io::Result<AppConfig> {
        Ok(serde_json::from_str(&fs::read_to_string(filepath).expect("Unable to read config"))?)
    }
}

impl AppConfig {
    fn create(self) -> std::io::Result<()> {
        let mut osconf = OSConfig::from_file(OSCONFIG_FILE_NAME.to_owned())?;
        if osconf.apps.contains(&self.name) {
            panic!("Already registered app {}", self.name);
        }
        osconf.apps.push(self.name.clone());

        let app_dir = format!("./{}.app", self.name.clone());

        fs::create_dir(app_dir)?;
        self.save()?;
        osconf.save()?;

        Ok(())
    }
}

impl fmt::Display for PlatformSpecific {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlatformSpecific::AVR(conf) => {
                write!(f, "    [AVR]:\n        F_CPU: {}\n        MCU: {}", conf.f_cpu, conf.mcu)
            }
        }
    }
}

impl fmt::Display for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data: String = format!("[AppConfig]\n    Name: {}\n    Provides: {}\n    Requiers apps: ", self.name, self.provides);
        if self.requiers_apps.len() == 0 {
            data = data + "None\n";
        }
        else {
            data = data + "\n";
            for i in &self.requiers_apps {
                data = data + "      - " + i + "\n";
            }
        }
        data = data + "    Requiers drivers: ";
        if self.requiers_apps.len() == 0 {
            data = data + "None\n";
        }
        else {
            data = data + "\n";
            for i in &self.requiers_drivers {
                data = data + "      - " + i;
            }
        }
        write!(f, "{}", data)
    }
}

impl Config<DriverConfig> for DriverConfig {
    fn empty() -> DriverConfig {
        DriverConfig{name: "".to_owned(), provides: "".to_owned(), requiers: Vec::new()}
    }

    fn save(self) -> std::io::Result<()> {
        save_config(&self, format!("./{}.driver/.driver.json", self.name.clone()))
    }

    fn from_file(filepath: String) -> std::io::Result<DriverConfig> {
        Ok(serde_json::from_str(&fs::read_to_string(filepath).expect("Unable to read config"))?)
    }
}

impl DriverConfig {
    fn create(self) -> std::io::Result<()> {

        let mut osconf = OSConfig::from_file(OSCONFIG_FILE_NAME.to_owned())?;
        if osconf.drivers.contains(&self.name) {
            panic!("Already registered driver {}", self.name);
        }
        osconf.drivers.push(self.name.clone());

        let driver_dir = format!("./{}.driver", self.name.clone());

        fs::create_dir(driver_dir)?;
        self.save()?;
        osconf.save()?;
        Ok(())
    }
}

impl fmt::Display for DriverConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data: String = format!("[DriverConfig]\n    Name: {}\n    Provides: {}\n    Requiers: ", self.name, self.provides);
        if self.requiers.len() == 0 {
            data = data + "None\n";
        }
        else {
            data = data + "\n";
            for i in &self.requiers {
                data = data + "      - " + i + "\n";
            }
        }
        write!(f, "{}", data)
    }
}

fn save_config<T>(conf: &T, filepath: String) -> std::io::Result<()> where T: Serialize {
    fs::write(filepath, serde_json::to_string(&conf)?)
}

fn download_distro() -> std::io::Result<()> {
    let res = match reqwest::blocking::get("https://github.com/niktob560/revos/tarball/master") {
        Ok(r) => r,
        e => panic!("{:?}", e)
    };
    let mut f = File::create("./.distro.tar.gz")?;
    f.write_all(&mut match res.bytes(){
        Ok(b) => b,
        e => panic!("{:?}", e)
    })?;
    f = File::open("./.distro.tar.gz")?;
    let tar = GzDecoder::new(f);
    let mut arc = Archive::new(tar);
    arc.unpack(".")?;
    fs::remove_file("./.distro.tar.gz")?;
    match fs::read_dir(".") {
        Ok(dirs) => {
            for path in dirs {
                fs::rename(format!("./{}", path.unwrap().file_name().to_str().unwrap()), "./.distro")?;
            }
        },
        err => panic!("{:?}", err)
    };
    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", HELP_STRING);
        return Ok(());
    }

    match args[1].as_ref() {
        "init" => {
            if args.len() < 3 {
                println!("Usage:\n\trock init [architecture] [args]");
                return Ok(());
            }
            let platform_config = match args[2].as_ref() {
                "avr" => {
                    if args.len() < 5 {
                        println!("Usage:\n\trock init avr [MCU] [F_CPU]");
                        return Ok(())
                    }
                    PlatformSpecific::AVR(AVRPlatformSpecificConfig{mcu: args[3].to_owned(), f_cpu: args[4].parse::<i32>().unwrap()})
                },
                _ => panic!("{}", HELP_STRING)
            };
            download_distro().expect("Unable to download distro");
            let conf = OSConfig{drivers: Vec::new(), apps: Vec::new(), platform_spec: platform_config};
            conf.save().expect("Unable to save OSConfig");
            match dir::copy("./.distro/base.app", "./", &CopyOptions::new()) {
                Ok(_) => (),
                _ => panic!("Failed to copy base.app")
            };
            for i in ["codecheck.sh", "Makefile"].iter() {
                match fs::copy(format!("./.distro/{}", i), format!("./{}", i)) {
                    Ok(_) => (),
                    _ => panic!("Unable to copy {}", i)
                }
            }
        },
        "create" => {

            if args.len() < 3 {
                println!("Usage:\n\trock create [app,driver] [args]");
                return Ok(());
            }

            match args[2].as_ref() {
                "app" => {
                    if args.len() < 4 {
                        println!("Usage:\n\trock create app [app name]");
                        return Ok(());
                    }
                    let app_name = args[3].to_owned();
                    let conf = AppConfig{name: app_name.clone(), provides: app_name.clone(), requiers_apps: Vec::new(), requiers_drivers: Vec::new()};
                    conf.create().expect("Unable to create app config");
                    fs::copy("./.distro/template.app/Makefile", format!("./{}.app/Makefile", app_name)).expect("Unable to init makefile");
                    fs::write(format!("./{}.app/main.c", app_name), format!("void {}()\n{}", app_name, "{\n\n}\n")).expect("Unable to init .c file");
                },
                "driver" => {
                    if args.len() < 4 {
                        println!("Usage:\n\trock create driver [driver name]");
                        return Ok(());
                    }
                    let driver_name = args[3].to_owned();

                    let conf = DriverConfig{name: driver_name.clone(), provides: driver_name.clone(), requiers: Vec::new()};
                    conf.create().expect("Unable init config file");

                    fs::copy("./.distro/gpio_c.driver/Makefile", format!("./{}.driver/Makefile", driver_name)).expect("Unable to init makefile");
                    fs::write(format!("./{}.driver/main.h", driver_name), format!("#ifndef __{}_H__\n#define __{}_H__\n\n\n#endif\n", driver_name.to_uppercase(), driver_name.to_uppercase())).expect("Unable to init .h file");
                    fs::write(format!("./{}.driver/main.c", driver_name), format!("#include \"main.h\"\n")).expect("Unable init .c file");
                },
                _ => {
                    println!("Usage:\n\trock create [app,driver] [args]");
                    return Ok(())
                }
            }
        },
        "show" => {
            if args.len() < 3 {
                let conf = OSConfig::from_file(OSCONFIG_FILE_NAME.to_owned())?;
                println!("{}", conf);
                return Ok(());
            }
            match args[2].as_ref() {
                "drivers" => {
                    for i in OSConfig::from_file(OSCONFIG_FILE_NAME.to_owned())?.drivers {
                        println!("{}", DriverConfig::from_file(format!("./{}.driver/.driver.json", i))?);
                    }
                },
                "apps" => {
                    for i in OSConfig::from_file(OSCONFIG_FILE_NAME.to_owned())?.apps {
                        println!("{}", AppConfig::from_file(format!("./{}.app/.app.json", i))?);
                    }
                },
                "app" => {
                    if args.len() < 4 { 
                        println!("Usage:\n\trock show app [app name]");
                        return Ok(());
                    }
                    println!("{}", AppConfig::from_file(format!("./{}.app/.app.json", args[3]))?);
                },
                "driver" => {
                    if args.len() < 4 {
                        println!("Usage:\n\trock show driver [driver name]");
                        return Ok(());
                    }
                    println!("{}", DriverConfig::from_file(format!("./{}.driver/.driver.json", args[3]))?);
                },
                _ => ()
            }
        },
        "build" => {
            fs::create_dir("./Build");
            // Create apps provided funcs defs
            let mut mods_src = format!("#ifndef __MODS_H__\n#define __MODS_H__\n");
            let osconf = OSConfig::from_file(OSCONFIG_FILE_NAME.to_owned())?;

            let funcs: Vec<String> = osconf.apps.iter().map(|s| {
                    let appconf = AppConfig::from_file(format!("./{}.app/.app.json", s)).expect(format!("Unable to read app config {}", s).as_str());
                    appconf.provides.clone()
                }).collect();
            
            let defs = funcs.iter().fold((format!(""), format!("")), |acc, s| {
                (acc.0 + "void " + s + "();\n", s.to_owned() + ", " + &acc.1)
            });

            let func_defs = defs.0;
            let task_list = defs.1;

            mods_src = mods_src + func_defs.as_str() + "\n#define __TASKS__ " + task_list.as_str() + "\n";
            mods_src = mods_src + "#endif";

            fs::write("Build/mods.h", mods_src)?;

            let mut proc = Command::new("make");
            let cmd = match osconf.platform_spec{
                PlatformSpecific::AVR(c) => {
                    proc.arg("-e").arg(format!("F_CPU={}", c.f_cpu)).arg("-e").arg(format!("MCU={}", c.mcu))
                }
            };
            cmd.status().expect("Failed to run make command");
        },
        "clean" => {
            Command::new("make").arg("clean").spawn()?;
        },
        "edit" => {
            if args.len() < 3 {
                println!("Usage:\n\trock edit [os,app,driver] [args]");
                return Ok(())
            }
            match args[2].as_ref() {
                "os" => {
                    println!("Nothing to edit yet");
                },
                "app" => {
                    if args.len() < 5 {
                        println!("Usage:\n\trock edit app [appname] [provides, requiers]");
                        return Ok(())
                    }
                    let app_name = args[3].clone();
                    let mut osconf = OSConfig::from_file(OSCONFIG_FILE_NAME.to_owned()).expect("Unable to read os config");
                    let mut appconf = AppConfig::from_file(format!("./{}.app/.app.json", app_name)).expect("Unable to read app config");
                    match args[4].as_ref() {
                        "provides" => {
                            if args.len() < 6 {
                                println!("Usage:\n\trock edit app [appname] provides [new value]");
                                return Ok(());
                            }
                            appconf.provides = args[5].clone();
                            appconf.save().expect("Unable to save app config");
                        },
                        "requiers" => {
                            if args.len() < 8 {
                                println!("Usage:\n\trock edit app [appname] requiers [apps,drivers] [add,remove] [name]");
                                return Ok(())
                            }
                            let (mut req_app, mut req_os) =
                                match args[5].as_ref() {
                                    "apps" => (appconf.requiers_apps.clone(), osconf.apps.clone()),
                                    "drivers" => (appconf.requiers_drivers.clone(), osconf.drivers.clone()),
                                    _ => panic!("Bad edit type"),
                                };

                            match args[6].as_ref() {
                                "add" => {
                                    if req_os.contains(&args[7]) {
                                        println!("Already registered {}", args[7].clone());
                                        return Ok(())
                                    }
                                    else {
                                        let distro_config = OSConfig::from_file(format!("./.distro/{}", OSCONFIG_FILE_NAME)).expect("Unable to read distro config");
                                        let distro_req = match args[5].as_ref() {
                                            "apps" => distro_config.apps.clone(),
                                            "drivers" => distro_config.drivers.clone(),
                                            _ => panic!("Bad edit type"),
                                        };
                                        if distro_req.contains(&args[7]) {
                                            req_app.push(args[7].clone());
                                            req_os.push(args[7].clone());
                                            let filename = args[7].clone() + "." + args[5].clone().replace("s", "").as_str();
                                            println!("fname: {}", filename);
                                            dir::copy(format!("./.distro/{}", filename), "./", &CopyOptions::new()).expect("Unable to copy");
                                        }
                                        else {
                                            println!("Unable to find {}", args[7].clone());
                                            return Ok(())
                                        }
                                    }
                                },
                                "remove" => {
                                    let name = args[7].clone();
                                    req_app.remove(req_app.iter().position(|x| *x == name).expect(format!("Provided {} not found", args[5].clone()).as_str()));
                                    req_os.remove(req_os.iter().position(|x| *x == name).expect(format!("Provided {} not found", args[5].clone()).as_str()));
                                },
                                _ => panic!("Bad action")
                            };
                            match args[5].as_ref() {
                                "apps" => {
                                    appconf.requiers_apps = req_app;
                                    osconf.apps = req_os;
                                },
                                "drivers" =>{
                                    appconf.requiers_drivers = req_app;
                                    osconf.drivers = req_os;
                                },
                                _ => panic!("Bad edit type"),
                            };
                            osconf.save().expect("Unable to save os config");
                            appconf.save().expect("Unable to save app config");
                        },
                        _ => println!("Usage:\n\trock edit app [appname] [provides|requiers_apps|requiers_drivers] [args]")
                    }
                },
                _ => println!("Usage:\n\trock edit [os,app,driver] [args]"),
            }
        },
        _ => panic!("{}", HELP_STRING)
    }
    Ok(())
}
