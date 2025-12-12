defmodule AOCSolvers.Day11 do
  def part1(input) do
    parse_graph(input)
    |> traverse("you")
    |> Enum.count()
  end

  defp traverse(nodes, root),
    do: traverse(nodes, root, MapSet.new())

  defp traverse(_, "out", visited) do
    if(MapSet.member?(visited, "dac") and MapSet.member?(visited, "fft")) do
      [visited]
    else
      []
    end
  end

  defp traverse(nodes, current_node, visited) do
    update = MapSet.put(visited, current_node)
    unvisited = Enum.reject(nodes[current_node], &MapSet.member?(visited, &1))

    for node <- unvisited,
        reduce: [] do
      paths ->
        traverse(nodes, node, update) ++ paths
    end
  end

  def part2(input) do
    parse_graph(input)
    |> traverse("svr")
    |> Enum.count()
  end

  defp parse_graph(input) do
    String.split(input, "\n", trim: true)
    |> Enum.reduce(%{}, fn line, acc ->
      [device_name, outputs] = String.split(line, ":")
      Map.put_new(acc, device_name, String.split(outputs))
    end)
  end
end
