#!/bin/bash
#Given the year, month, and day, return the day of the week.
#cerner_2^5_2019
# NOTE THIS WILL HAVE ISSUES WITH  TRYING TO FIND THE DATE AFTER 1-20-2038 03:14:07 UTC on any processor using 32bit signed binary integer for time
#Docuemntation for bug: https://en.wikipedia.org/wiki/Year_2038_problem
# Relevant XKCD: https://xkcd.com/607/
echo "When entering date please enter in MM-DD-YYYY format"
read -p "Please enter the month (MM)" MM
read -p "Please enter the day of the week (DD)" DD
read -p "Please enter the year(YYYY)"  YYYY

echo "$YYYY, $MM, $DD"
date +%A -d$YYYY$MM$DD
#Test Options
#echo "2017, 10, 30"
#date +%A -d20171030
#echo "2016, 2, 29"
#date +%A -d20160229
#echo "2015, 2, 28"
#date +%A -d20150228
#echo "29, 4, 12"
#date +%A -d00290412
#echo "570, 11, 30"
#date +%A -d05701130
#echo "1066, 9, 25"
#date +%A -d10660925
#echo "1066, 10, 14"
#date +%A -d10661014
#echo "1776, 7, 4"
#date +%A -d17760704
#echo "1933, 1, 30"
#date +%A -d19330130
#echo "1953, 3, 6"
#date +%A -d19530306
#echo "2100, 1, 9"
#date +%A -d21000109
#echo "2202, 12, 15"
#date +%A -d22021215
#echo "7032, 3, 26"
#date +%A -d70320326
#echo "1616, 4, 23"
#date +%A -d16160423
#echo "2038, 1, 20"
#date +%A -d20380120
