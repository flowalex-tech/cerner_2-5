#!/bin/bash
# Given a string containing only the characters `x` and `y`, find whether there are the same number of `x`'s and `y`'s.
# cerner_2^5_2019
read -rp "Please enter your string: " balchars
#converts string input to an array
read -a char_array <<< $(echo "$balchars" | sed 's/./& /g')
x=0
y=0
for i in "${char_array[@]}"
 do
  if [[ $i == 'x' ]]; then
    x=$x+1
  elif [[ $i == 'y' ]]; then
    y=$y+1
  else
    echo "invalid entry: $i "
  fi
done
if [[ $x = $y ]]; then

echo "True"

else

echo "False"

fi

exit 0
