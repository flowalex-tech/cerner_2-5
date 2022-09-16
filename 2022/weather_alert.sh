#!/bin/bash
# cerner_2tothe5th_2022
# Simple script that calls the NWS api for alerts and uses jq to formay the respsonse
#MNZ012 Northern MN 
#MNZ060 Hennipen County
 curl https://api.weather.gov/alerts/active/zone/$1 | jq '.features[1].properties | .senderName, .headline, .description, .areaDesc, .effective, .expires'
