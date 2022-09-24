#!/bin/bash
# cerner_2tothe5th_2022

# shellcheck disable=SC2086,SC2162
# Given a text file file.txt that contains list of phone numbers (one per line), write a one liner bash script to print all valid phone numbers.
#You may assume that a valid phone number must appear in one of the following two formats: (xxx) xxx-xxxx or xxx-xxx-xxxx. (x means a digit)
# You may also assume each line in the text file must not contain leading or trailing white spaces.
#
#Example:
#
#Assume that file.txt has the following content:
#
#987-123-4567
#123 456 7890
#(123) 456-7890
#Your script should output the following valid phone numbers:
#
#987-123-4567
#(123) 456-7890
#From https://leetcode.com/problems/valid-phone-numbers/
# Regex found here http://regexlib.com/Search.aspx?k=phone&AspxAutoDetectCookieSupport=1
regex="^((\([0-9]{3}\) )|([0-9]{3}\-))[0-9]{3}\-[0-9]{4}$"

while read LINE 
do
    if  [[ LINE =~ $regex ]]
    then 
        echo $LINE
    fi
done < file.txt
