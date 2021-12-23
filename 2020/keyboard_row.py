#!/usr/bin/env python3
# cerner_2^5_2020
"""
Given a List of words, return the words that can be typed using letters of alphabet on only one row's of American keyboard like the image below.

Example:
Input: ["Hello", "Alaska", "Dad", "Peace"]
Output: ["Alaska", "Dad"]

Note:
    You may use one character in the keyboard more than once.
    You may assume the input string will only contain letters of alphabet.
"""

row1 = set("qwertyuiop")
row2 = set("asdfghjkl")
row3 = set("zxcvbnm")

words = ["Hello", "Alaska", "Dad", "Peace", "qwerty", "top"]

rows = ({'t','q','p','i','w','r','o','u','e','y'},
        {'a','s','d','f','g','h','j','k','l'},
        {'z','x','c','v','b','n','m'})

wordArray = []

for i in words:
    letters = set(i.lower())
    for row in rows:
        if letters.issubset(row):
            wordArray.append(i)
            break
print(wordArray)
