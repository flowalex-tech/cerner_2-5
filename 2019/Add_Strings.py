#Tested and working with python 3
# cerner_2-5_2019
# Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.
num1_in = input("Enter your first number: ")
# print(num1_in)
num2_in = input("Enter your second number: ")
# print(num2_in)
num1 = list(num1_in)
num2 = list(num2_in)
if len(num1) < len(num2):
    for i in range(len(num2) - len(num1)):
        num1.insert(0, "0")
elif len(num1) > len(num2):
    for i in range(len(num1) - len(num2)):
        num2.insert(0, "0")

ans = 0
for i in range(len(num1)):
    precalculated = (ord(num1.pop()) + ord(num2.pop())) % 48
    ans += precalculated * 10 ** i
    print(ans)
