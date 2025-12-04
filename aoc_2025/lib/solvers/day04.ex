defmodule AOC.Solvers.Day04 do
  @adjacent MapSet.new([
              {0, 1},
              {0, -1},
              {1, 0},
              {-1, 0},
              {-1, -1},
              {1, 1},
              {1, -1},
              {-1, 1}
            ])

  def part1(input) do
    map = input |> to_map_set()
    map |> Enum.filter(fn coord -> reachable?(map, coord) end) |> Enum.count()
  end

  def part2(input) do
    input |> to_map_set() |> count_removable()
  end

  defp count_removable(map), do: count_removable(map, 0)

  defp count_removable(map, count) do
    removable =
      map
      |> Enum.filter(fn coord -> reachable?(map, coord) end)
      |> MapSet.new()

    case MapSet.size(removable) do
      0 -> count
      n -> MapSet.difference(map, removable) |> count_removable(count + n)
    end
  end

  defp reachable?(map, {row, col}) do
    n_neighbors =
      @adjacent
      |> Enum.filter(fn {dr, dc} ->
        MapSet.member?(map, {row + dr, col + dc})
      end)
      |> Enum.count()

    n_neighbors < 4
  end

  defp to_map_set(input) do
    grid =
      input
      |> String.split("\n")
      |> Enum.map(&String.graphemes/1)

    for {row, row_idx} <- Enum.with_index(grid),
        {_, col_idx} <-
          Enum.with_index(row) |> Enum.filter(fn {v, _} -> v === "@" end),
        into: MapSet.new() do
      {row_idx, col_idx}
    end
  end
end
