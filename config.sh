#!/bin/bash

set -e
echo -e '\033[1;32m'"Running config\033[0m"

apps=$(ls | grep '\.app')
incs=$(echo $apps | tr ' ' '\n' | sed 's/\.app$/.h/g' | sed 's/.*./#include "&"\n/g')

tasks=$(cat */*.h | grep 'void\s__[a-zA-Z_\-]*[a-zA-Z_\-]_app_task__();' | grep -o '__[a-zA-Z_\-]*[a-zA-Z_\-]__' | sed 's/.*./&,/g' | tr '\n' ' ' | sed 's/, $//g')

#save ifndef
cat base.app/.base.h.old | head -2 > base.app/base.h

#save system includes
cat base.app/.base.h.old | tail -n +2 | grep '#include ' >> base.app/base.h

ignore_lines=$(($(wc -l base.app/base.h | grep -o '[0-9]*[0-9]') + 1))

echo -e $incs | sed 's/" #/"\n#/g' >> base.app/base.h

tail -n +$ignore_lines base.app/.base.h.old | sed "s/__put__task__list__here__/$tasks/g" >> base.app/base.h

echo -e '\033[1;32m'"Config done\033[0m"

set +e