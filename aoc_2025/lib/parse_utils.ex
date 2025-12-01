defmodule AdventOfCode.ParseUtils do
  @moduledoc """
  Provides utilities for parsing Advent of Code input.
  """

  @doc """
  Splits the input into a list of trimmed lines.

  ## Examples

      iex> InputParser.lines("line1\\nline2\\nline3")
      ["line1", "line2", "line3"]

  """
  def lines(input), do: String.split(input, "\n", trim: true)

  @doc """
  Splits the input into a list of token lists, where tokens are separated by whitespace.

  ## Examples

      iex> InputParser.tokens("a b c\\nd e f\\ng h i")
      [["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]]

  """
  def tokens(input), do: input |> lines() |> Enum.map(&String.split/1)
end
