#!/usr/bin/env bash
# Challenge

#The input is a single number: the number at which the game starts. Write a program that plays the Threes game, and outputs a valid sequence of steps you need to take to get to 1. Each step should be output as the number you start at, followed by either -1 or 1 (if you are adding/subtracting 1 before dividing), or 0 (if you are just dividing). The last line should simply be 1.
#cerner_2^5_2019

read -p "Please enter your value: " num
while [ $((num)) -ne 1 ]
do

  check=$(( num % 3 ))
  check2=$(( (num + 1) % 3 ))
    if [ $((check)) -eq 0 ]
      then
        echo "$num 0"
        num=$(( num / 3 ))
    elif [ "$check2" -eq 0 ]
      then
          echo "$num +1"
num=$(( (num+1) / 3 ))
    else
      echo "$num -1"
      num=$(( (num-1) / 3 ))

    fi

done

echo "1"
