#!/bin/bash

cd /home/pipi/insertdb;
git pull;
cargo run --release -- /media/pipi/0123-4567/Master_Split;
cargo run --release -- /media/pipi/taz/Master_Split;
cargo run --release -- /media/pipi/USB01/Master_Split;

cd /media/pipi/0123-4567/JPG;
ls -l | wc -l;
cd /media/pipi/taz/JPG;
ls -l | wc -l;
cd /media/pipi/USB01/JPG;
ls -l | wc -l;
echo "-------------------";
cd /media/pipi/0123-4567/Master_Split;
ls -l | wc -l;
cd /media/pipi/taz/Master_Split;
ls -l | wc -l;
cd /media/pipi/USB01/Master_Split;
ls -l | wc -l;

