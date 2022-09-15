# cerner_2tothe5th_2022
# Doing the good old fizzbuzz since I don't have time today to create something better
for i in range(1, 101):
  if i % 15 == 0:
    print( str(i) + ' FIZZBUZZ')
  elif i % 3 == 0:
    print( str(i) + ' FIZZ')
  elif i % 5 == 0:
    print( str(i) + ' BUZZ')
  else:
    print(str(i))
