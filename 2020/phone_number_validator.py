#!/usr/bin/env python3
# cerner_2^5_2020
# Check that a phone number is a valid number on an exchange
#From https://leetcode.com/problems/valid-phone-numbers/ but that version only requres they are in the valid format

import phonenumbers
import re

phone_file = open("file.txt", "r")

for i in phone_file:
    phone = phonenumbers.parse(i, "US")
    if phonenumbers.is_valid_number(phone):
        print(i)
            
phone_file.close
