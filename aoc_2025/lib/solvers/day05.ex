defmodule AOC.Solvers.Day05 do
  def part1(input) do
    [first, last] =
      input
      |> String.split("\n\n")
      |> Enum.map(&String.split(&1, "\n"))

    ranges = Enum.map(first, &to_range/1)
    ids = Enum.map(last, &String.to_integer/1)

    Enum.count(ids, fn id ->
      Enum.any?(ranges, &Enum.member?(&1, id))
    end)
  end

  def part2(input) do
    [first | _] = String.split(input, "\n\n")

    first
    |> String.split("\n")
    |> Enum.map(&to_range/1)
    |> reconciled_size()
  end

  defp reconciled_size(ranges) do
    [head | tail] = Enum.sort_by(ranges, & &1.first)

    {_, size} =
      Enum.reduce(tail, {head, 0}, fn next, {current, size} ->
        if Range.disjoint?(current, next) do
          {Range.new(current.first, max(current.last, next.last)), size}
        else
          {next, size + Range.size(current)}
        end
      end)

    size
  end

  defp to_range(line) do
    String.split(line, "-")
    |> Enum.map(&String.to_integer(&1))
    |> then(fn [first, last] -> first..last end)
  end
end
