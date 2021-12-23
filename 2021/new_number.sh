#!/bin/bash
# cerner_2tothe5th_2021
n=1
for i in {1..64}; do
  n=$(echo $n | ./add)
done
echo $n
