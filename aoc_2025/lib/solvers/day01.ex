defmodule AdventOfCode.Solutions.Day01 do
  defp parse_input(input) do
    input
    |> String.replace(~r/L|R/, fn
      "L" -> "-"
      "R" -> ""
    end)
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  defp solve(input, reducer) do
    {_, n_zeros} =
      parse_input(input)
      |> Enum.reduce({50, 0}, reducer)

    n_zeros
  end

  def part1(input) do
    solve(input, fn offset, {current, count} ->
      next = Integer.mod(current + offset, 100)
      {next, count + if(next == 0, do: 1, else: 0)}
    end)
  end

  def part2(input) do
    solve(input, fn offset, {current, count} ->
      next_raw = current + offset
      crossings = abs(div(next_raw, 100)) + if current != 0 && next_raw <= 0, do: 1, else: 0
      {Integer.mod(next_raw, 100), count + crossings}
    end)
  end
end
