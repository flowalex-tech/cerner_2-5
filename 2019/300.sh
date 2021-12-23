#!/usr/bin/env bash
#If you wrote all the numbers from 300 to 400 on a piece of paper, how many times would you have written the number 3?
#cerner_2^5_2019
echo "Counting all numbers from 300 to 400"
for i in {300..400}
do
	echo $i >> num.tmp
	echo $i
	sleep .25s
done
clear
echo "Finding all entries that contain the number 3"
cat num.tmp | awk '/[3][0-9][0-9]/' >> entries.tmp
cat num.tmp | awk '/[3][3][0-9]/' >> entries.tmp
cat num.tmp | awk '/[3][0-9][3]/' >> entries.tmp
file="entries.tmp"
cat $file  | while read LINE
	do
echo -e "\e[5m$LINE\e[5m"
sleep .25s
	done

wc -l entries.tmp
rm entries.tmp
rm num.tmp
