#!/bin/bash
# shellcheck disable=SC2004
# cerner_2tothe5th_2022

line_count=$(wc -l ./file.txt | awk -F " " '{print $1}')

line_count=$((line_count +1))

if (($line_count < 10)) 
then
    echo "File has less than 10 lines"
else
    sed '10q;d' ./file.txt # Found the sed command here: https://stackoverflow.com/questions/6022384/bash-tool-to-get-nth-line-from-a-file I was trying a complicated for loop with awk
fi
