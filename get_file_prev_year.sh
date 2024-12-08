#!/bin/bash

day=$1
year=$2
cookie=`cat .aoc_cookie`

if [ "$day" = "" ]; then
    echo "Missing day"
    exit 1
elif [ "$year" = "" ]; then
    echo "Missing year"
    exit 3
elif [ "$cookie" = "" ]; then
    echo "Missing cookie"
    exit 2
fi

mkdir -p data/$year
mkdir data/$year/day$day
touch data/$year/day$day/test.txt
curl --cookie "$cookie" https://adventofcode.com/$year/day/$day/input > data/$year/day$day/input.txt
