defmodule AOC.Solvers.Day08 do
  defmodule Point do
    defstruct [:x, :y, :z]

    def distance(
          %Point{x: x1, y: y1, z: z1},
          %Point{x: x2, y: y2, z: z2}
        ) do
      :math.sqrt(
        :math.pow(x2 - x1, 2) + :math.pow(y2 - y1, 2) + :math.pow(z2 - z1, 2)
      )
    end
  end

  defmodule CircuitConnector do
    defstruct [:circuit_mapping, :direct_connections, :neighbor_lists]

    def new(boxes) do
      mapping =
        for {box, index} <- Enum.with_index(boxes),
            into: %{},
            do: {box, index}

      neighbor_lists =
        Enum.map(boxes, fn box ->
          sorted_neighbors =
            boxes
            |> List.delete(box)
            |> Enum.sort_by(&Point.distance(box, &1))

          {box, sorted_neighbors}
        end)
        |> sort_by_first_neighbor()

      %CircuitConnector{
        circuit_mapping: mapping,
        direct_connections: Map.new(Enum.map(boxes, &{&1, MapSet.new()})),
        neighbor_lists: neighbor_lists
      }
    end

    def connect_n_times(connector, 0), do: connector

    def connect_n_times(connector, lights_left) do
      %CircuitConnector{
        neighbor_lists: [{a, [b | _]} | _],
        direct_connections: conns
      } = connector

      connector
      |> connect()
      |> connect_n_times(
        if b in conns[a], do: lights_left, else: lights_left - 1
      )
    end

    def almost_connect_all(circuit_connector) do
      %CircuitConnector{circuit_mapping: m, neighbor_lists: [{a, [b | _]} | _]} =
        circuit_connector

      n_circuits = Map.values(m) |> Enum.uniq() |> Enum.count()

      if n_circuits == 1 do
        {a, b}
      else
        circuit_connector
        |> connect()
        |> almost_connect_all()
      end
    end

    def connect(%CircuitConnector{
          neighbor_lists: [{a, [b | c]} | d],
          circuit_mapping: mapping,
          direct_connections: connections
        }) do
      if b in connections[a] do
        %CircuitConnector{
          circuit_mapping: mapping,
          direct_connections: connections,
          neighbor_lists: sort_by_first_neighbor([{a, c} | d])
        }
      else
        a_circuit_id = mapping[a]
        b_circuit_id = mapping[b]

        updated_mapping =
          for {box, id} <- mapping, into: %{} do
            if id == b_circuit_id, do: {box, a_circuit_id}, else: {box, id}
          end

        updated_connections =
          connections
          |> Map.update!(a, &MapSet.put(&1, b))
          |> Map.update!(b, &MapSet.put(&1, a))

        %CircuitConnector{
          circuit_mapping: updated_mapping,
          direct_connections: updated_connections,
          neighbor_lists: sort_by_first_neighbor([{a, c} | d])
        }
      end
    end

    defp sort_by_first_neighbor(neighbor_lists) do
      Enum.sort_by(neighbor_lists, fn
        {box, [nearest | _]} -> Point.distance(box, nearest)
      end)
    end
  end

  def part1(input) do
    %CircuitConnector{circuit_mapping: connected_circuits} =
      parse_points(input)
      |> CircuitConnector.new()
      |> CircuitConnector.connect_n_times(1000)

    circuit_product(connected_circuits)
  end

  def part2(input) do
    {a, b} =
      parse_points(input)
      |> CircuitConnector.new()
      |> CircuitConnector.almost_connect_all()

    a.x * b.x
  end

  def parse_points(input) do
    String.split(input, "\n", trim: true)
    |> Enum.map(fn line ->
      [x, y, z] = String.split(line, ",") |> Enum.map(&String.to_integer/1)
      %Point{x: x, y: y, z: z}
    end)
  end

  def circuit_product(circuits) do
    circuits
    |> Enum.group_by(fn {_, id} -> id end)
    |> Enum.map(fn {_id, boxes} -> length(boxes) end)
    |> Enum.sort(:desc)
    |> Enum.take(3)
    |> Enum.product()
  end
end
