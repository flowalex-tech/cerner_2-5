# cerner_2^5_2020
# Simple elixir script that gets the word count from a file

filename = IO.gets("File to count words from: ") |> String.trim

words = String.split(File.read!(filename), ~r{(\\n|[^\w'])+})
  |> Enum.filter(fn x -> x !=  "" end)

IO.inspect(words)
Enum.count(words) |>  IO.puts()
