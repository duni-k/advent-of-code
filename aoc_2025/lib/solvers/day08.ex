defmodule AOC.Solvers.Day08 do
  defmodule JunctionBox do
    defstruct [:x, :y, :z]

    def distance(
          %JunctionBox{x: x1, y: y1, z: z1},
          %JunctionBox{x: x2, y: y2, z: z2}
        ) do
      :math.sqrt(
        :math.pow(x2 - x1, 2) + :math.pow(y2 - y1, 2) + :math.pow(z2 - z1, 2)
      )
    end
  end

  def part1(input) do
    circuits =
      String.split(input, "\n", trim: true)
      |> Enum.with_index()
      |> Enum.reduce(Map.new(), fn {line, i}, acc ->
        [x, y, z] = String.split(line, ",") |> Enum.map(&String.to_integer/1)
        Map.put(acc, %JunctionBox{x: x, y: y, z: z}, i)
      end)

    boxes = Map.keys(circuits)

    combinations =
      for a <- boxes,
          b <- boxes,
          a != b,
          do: [a, b]

    initial_conns = Enum.map(boxes, &{&1, []}) |> Map.new()

    {circuits, _} =
      Enum.reduce(1..1000, {circuits, initial_conns}, fn _, state ->
        connect_closest(state, combinations)
      end)

    circuits
    |> Enum.group_by(fn {_, id} -> id end)
    |> Enum.sort_by(fn {_, boxes} -> length(boxes) end, :desc)
    |> Enum.take(3)
    |> Enum.map(fn {_id, boxes} -> length(boxes) end)
    |> Enum.product()
  end

  defp connect_closest({circuits, connections}, combinations) do
    [a, b] =
      combinations
      |> Enum.reject(fn [a, b] -> b in connections[a] end)
      |> Enum.min_by(fn [a, b] -> JunctionBox.distance(a, b) end)

    connect(circuits, connections, a, b)
  end

  def connect(circuits, connections, a, b) do
    a_circuit = circuits[a]
    b_circuit = circuits[b]

    updated_circuits =
      Enum.map(circuits, fn {box, id} ->
        if id == b_circuit do
          {box, a_circuit}
        else
          {box, id}
        end
      end)
      |> Map.new()

    updated_connections =
      connections
      |> Map.put(a, [b | connections[a]])
      |> Map.put(b, [a | connections[b]])

    {updated_circuits, updated_connections}
  end
end
