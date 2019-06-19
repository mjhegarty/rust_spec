#!/bin/bash 

#script to generate a spectrogram and display it on screen

echo "generating a spectrogram of data collected by rtl_sdr"
rm text.txt data.txt
cargo run

FILENAME="test.txt" 
sed 's/[][]//g' <$FILENAME  | tr , '\n' >"data.txt"

python spec_viewer.py data.txt
