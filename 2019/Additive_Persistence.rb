# Inspired by this tweet: https://twitter.com/fermatslibrary/status/1089883307473543170, today's challenge is to calculate the additive persistence http://mathworld.wolfram.com/AdditivePersistence.html of a number, defined as how many loops you have to do summing its digits until you get a single digit number. Take an integer N:
def persistance(num)
  count = 0
  until num < 10
    num = num.digits(10).sum
    count += 1
  end
  puts count
end
puts 'Please enter a number'
number = gets.chomp.to_i
printf(number, persistance(number))
# in a different version of the script we tried  19_999_999_999_999_999_999_999, unfortunetly the computer we were running it on ran out of memory and the script exited without finishing
# cerner_2^5_2019
