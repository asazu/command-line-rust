#!/usr/bin/env bash

FILE="tests/inputs/cant-touch-this"

touch $FILE
chmod 000 $FILE

echo $FILE > .gitignore