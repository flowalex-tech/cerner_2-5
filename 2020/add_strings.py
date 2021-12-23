#!/usr/bin/env python3
# cerner_2^5_2020

"""Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.
Note:
The length of both num1 and num2 is < 5100.
Both num1 and num2 contains only digits 0-9.
Both num1 and num2 does not contain any leading zero.
You must not use any built-in BigInteger library or convert the inputs to integer directly.
From: https://leetcode.com/problems/add-strings/
"""

num1_in = input("Enter your first number: ")
num2_in = input("Enter your second number: ")

num1 = list(num1_in)
num2 = list(num2_in)

if len(num1) < len(num2):
    for i in range(len(num2)-len(num1)):
        num1.insert(0,"0")
elif len(num1) > len(num2):
    for i  in range(len(num1)-len(num2)):
        num2.insert(0,"0")

ans = 0

for i  in range(len(num1)):
    precalulated = (ord(num1.pop()) + ord.(num2.pop())) % 48
    ans += precalulated * 10 ** i 
    print ans
