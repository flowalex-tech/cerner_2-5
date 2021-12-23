#!/usr/bin/env bash
# Given three integers between 0 and 255, corresponding to the red, green, and blue channel values of a color, find the hex string for that color. You may use anything built into your programming language, such as for base conversion, but you can also do it manually.
# cerner_2^5_2019
# NOTE Requires sotware bc to be installed
# Man page for bc https://ss64.com/bash/bc.html
#Debian based: sudo apt-get install bc
# RHEL based: sudo yum install bc
echo "Enter the rgb value putting a space between values."
read redn greenn bluen
redhex=`echo "ibase=10;obase=16;$redn"|bc`
greenhex=`echo "ibase=10;obase=16;$greenn"|bc`
bluehex=`echo "ibase=10;obase=16;$bluen"|bc`
echo "The hexadecimal value is #$redhex$greenhex$bluehex"
