#!/bin/sh
set -e
# Any subsequent commands which fail will cause the shell script to exit immediately

if [ -z "$1" ]
  then
    echo "Supply path of eagle directory. i.e. : ./copy_schematic.sh /path/to/eagle/dir"
    exit 1
fi

SOURCE=$1

cp $SOURCE"schematic.sch" .
cp $SOURCE"schematic.brd" .
cp $SOURCE"eagle.epf" .

echo "eagle files copied from "$SOURCE