defmodule Greeter do
# cerner_2^5_2020
# Simple program that greets you and will give a different response if your name is Alex
  def start do
    name = IO.gets("Hi there! What's your name?\n")

    case String.trim(name) do
      "Alex" ->
        IO.puts("Wow! Alex is my favorite name. I was programmed by someone named Alex!")
      name ->
        "Hi, #{name}. It's nice to meet you."
    end
  end
end
