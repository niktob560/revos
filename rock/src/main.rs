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

#[derive(Debug, Deserialize, Serialize)]
struct OSConfig {
	drivers: Vec<String>,
	apps: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    name: String,
    provides: String,
    requiers_apps: Vec<String>,
    requiers_drivers: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
struct DriverConfig {
    name: String,
    provides: String,
    requiers: Vec<String>
}

impl OSConfig {
    fn empty() -> OSConfig {
        OSConfig{drivers: Vec::new(), apps: Vec::new()}
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
            data = data + "      - " + i;
        }
        write!(f, "{}", data)
    }
}

impl AppConfig {
    fn empty(name: String) -> AppConfig {
        AppConfig{name: name.clone(), provides: name.clone(), requiers_apps: Vec::new(), requiers_drivers: Vec::new()}
    }

    fn save(self) -> std::io::Result<()> {
        save_config(&self, format!("./{}.app/.app.json", self.name.clone()))
    }

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

    fn from_file(filepath: String) -> std::io::Result<AppConfig> {
        Ok(serde_json::from_str(&fs::read_to_string(filepath).expect("Unable to read config"))?)
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

impl DriverConfig {
    fn empty(name: String) -> DriverConfig {
        DriverConfig{name: name.clone(), provides: name.clone(), requiers: Vec::new()}
    }

    fn save(self) -> std::io::Result<()> {
        save_config(&self, format!("./{}.driver/.driver.json", self.name.clone()))
    }

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

    fn from_file(filepath: String) -> std::io::Result<DriverConfig> {
        Ok(serde_json::from_str(&fs::read_to_string(filepath).expect("Unable to read config"))?)
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
            download_distro()?;
            OSConfig::empty().save()?;
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
                    let conf = AppConfig::empty(app_name.clone());
                    conf.create()?;
                    println!("Creating app {}", app_name);
                },
                "driver" => {
                    if args.len() < 4 {
                        println!("Usage:\n\trock create driver [driver name]");
                        return Ok(());
                    }
                    let driver_name = args[3].to_owned();

                    let conf = DriverConfig::empty(driver_name.clone());
                    conf.create()?;

                    println!("Done driver {}", driver_name);
                },
                _ => println!("Usage:\n\trock create [app,driver] [args]")
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
            fs::create_dir("./Build")?;
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

            Command::new("make").spawn()?;
        },
        "clean" => {
            Command::new("make").arg("clean").spawn()?;
        },
        _ => println!("{}", HELP_STRING)
    }
    Ok(())
}
