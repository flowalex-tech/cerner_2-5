#Given an array of size n, find the majority element. The majority element is the element that appears more than ⌊ n/2 ⌋ times.
# cerner_2^5_2019 
nums=[3,2,3]
#nums=[2,2,1,1,1,2,2]
for num in set(nums):
    if nums.count(num) > len(nums) / 2:
        print(num)
        
# RESULTS     
# /usr/local/bin/python3 /Users/aw062641/git/python/week37.py
# 2
# Process finished with exit code 0
# /usr/local/bin/python3 /Users/aw062641/git/python/week37.py
# 3
# Process finished with exit code 0
