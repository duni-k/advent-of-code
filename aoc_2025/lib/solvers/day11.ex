defmodule AOCSolvers.Day11 do
  def part1(input) do
    parse_graph(input)
    |> traverse("you")
    |> Enum.count()
  end

  defp traverse(graph, root),
    do: traverse(graph, root, MapSet.new())

  defp traverse(_, "out", visited), do: visited

  defp traverse(nodes, current_node, visited) do
    IO.inspect(current_node, label: "Current Node")
    IO.inspect(nodes, label: "Nodes")

    update =
      MapSet.put(visited, current_node) |> IO.inspect(label: "Visited Nodes")

    for node <- nodes[current_node],
        not MapSet.member?(visited, node),
        reduce: [] do
      paths ->
        traverse(nodes, node, update) ++ paths
    end
  end

  def part2(input) do
    parse_graph(input)
    |> traverse("svr")
    |> IO.inspect()
    |> Enum.filter(fn path ->
      MapSet.member?(path, "dac") and MapSet.member?(path, "fft")
    end)
    |> Enum.count()
  end

  defp parse_graph(input) do
    String.split(input, "\n", trim: true)
    |> Enum.reduce(%{}, fn line, acc ->
      [device_name, outputs] = String.split(line, ":")
      Map.put(acc, device_name, String.split(outputs))
    end)
  end
end
