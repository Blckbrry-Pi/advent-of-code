#!/bin/bash

day=$1
cookie=`cat .aoc_cookie`

if [ "$day" = "" ]; then
    echo "Missing day"
    exit 1
elif [ "$cookie" = "" ]; then
    echo "Missing cookie"
    exit 2
fi

mkdir data/day$day
touch data/day$day/test.txt
curl --cookie "$cookie" https://adventofcode.com/2024/day/$day/input > data/day$day/input.txt

