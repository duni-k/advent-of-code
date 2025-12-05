defmodule AOC.Solvers.Day05 do
  def part1(input) do
    [first, last] =
      input
      |> String.split("\n\n")

    ranges =
      first
      |> String.split("\n")
      |> Enum.map(&to_range/1)

    last
    |> String.split("\n")
    |> Enum.map(&String.to_integer(&1))
    |> Enum.filter(&in_any_range?(&1, ranges))
    |> Enum.count()
  end

  def part2(input) do
    ranges =
      input
      |> String.split("\n\n")
      |> then(fn [first, _] -> first end)
      |> String.split("\n")
      |> Enum.map(&to_range/1)

    ranges
    |> Enum.reduce(MapSet.new(), fn range, viable_ids ->
      MapSet.union(viable_ids, MapSet.new(range))
    end)
    |> MapSet.size()
  end

  defp in_any_range?(id, ranges) do
    Enum.any?(ranges, fn range -> Enum.member?(range, id) end)
  end

  defp to_range(string) do
    string
    |> String.split("-")
    |> Enum.map(&String.to_integer(&1))
    |> then(fn [first, second] -> first..second end)
  end
end
