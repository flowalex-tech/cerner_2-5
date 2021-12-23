defmodule GuessingGame do
  # cerner_2^5_2020
  # Simple program that guesses a number by asking if it is higher or lower
  # Makes sure that the low number is never larger than high
  def guess(a, b) when a > b, do: guess(b, a)
  # The guess uses a case statement to ask if bigger/smaller/correct
  def guess(low, high) do
    # what the guess is
    answer = IO.gets("Hmm... Maybe you are thinking of #{mid(low, high)}?\n")
    # user respone to the guess
    case String.trim(answer) do
      "bigger" ->
        bigger(low, high)
      "smaller" ->
        smaller(low, high)
      "yes" ->
        "I was able to guess your number"
      _ ->
        IO.puts( ~s{Type "bigger" if the number is larger, "smaller" if the number is smaller, or "yes" if the number was correct})
        guess(low, high)
    end
  end
  # gets the middle of the low and high number for the guess
  def mid(low, high) do
    div(low + high, 2)
  end
  # Creates a new low based off the first guess
  def bigger(low, high) do
    new_low = min(high, mid(low, high) + 1)
    guess(new_low, high)
  end
  # Creates a new high based off the first guess
  def smaller(low, high) do
    new_high = max(low, mid(low, high) - 1)
    guess(low, new_high)
  end
end
