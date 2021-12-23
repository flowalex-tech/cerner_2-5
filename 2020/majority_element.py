#!/usr/bin/env python3
# cerner_2^5_2020
"""
Given an array of size n, find the majority element. The majority element is the element that appears more than ⌊ n/2 ⌋ times.
You may assume that the array is non-empty and the majority element always exist in the array.

Example 1:
Input: [3,2,3]
Output: 3

Example 2:
Input: [2,2,1,1,1,2,2]
Output: 2
From: https://leetcode.com/problems/majority-element/
"""

nums=[3,2,3]
#nums=[2,2,1,1,1,2,2]

for num in set(nums):
    if nums.count(num) > len(nums) / 2:
        print(num)
