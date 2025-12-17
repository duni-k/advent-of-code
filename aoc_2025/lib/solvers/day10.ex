defmodule AOCSolvers.Day10 do
  defmodule Machine do
    import Bitwise

    defstruct [
      :buttons,
      :indicators,
      :joltage_requirements,
      :current_joltage,
      lights: 0
    ]

    def from_string(input) do
      for line <- String.split(input, "\n", trim: true) do
        [raw_light | raw_buttons] = String.split(line)
        [joltage_reqs | raw_buttons] = Enum.reverse(raw_buttons)

        buttons =
          for raw_button <- Enum.reverse(raw_buttons),
              do:
                String.trim_leading(raw_button, "(")
                |> String.trim_trailing(")")
                |> String.split(",")
                |> Enum.map(&String.to_integer/1)
                |> MapSet.new()

        indicators =
          String.trim_leading(raw_light, "[")
          |> String.trim_trailing("]")
          |> String.to_charlist()
          |> Enum.with_index()
          |> Enum.reduce(0, fn {ch, idx}, acc ->
            if ch == ?#, do: acc ||| 1 <<< idx, else: acc
          end)

        joltage_reqs =
          String.trim_leading(joltage_reqs, "{")
          |> String.trim_trailing("}")
          |> String.split(",")
          |> Enum.map(&String.to_integer/1)
          |> Enum.with_index()
          |> Enum.map(fn {n, idx} -> {idx, n} end)
          |> Map.new()

        %Machine{
          indicators: indicators,
          buttons: buttons,
          joltage_requirements: joltage_reqs,
          current_joltage:
            Enum.map(joltage_reqs, fn {idx, _} -> {idx, 0} end) |> Map.new()
        }
      end
    end

    def shortest_button_sequence(machine) do
      bfs([{machine.lights, 0, []}], MapSet.new(), machine)
    end

    defp bfs([], _visited, _machine), do: nil

    defp bfs([{lights, depth, path} | queue], visited, machine) do
      new_states =
        for button <- machine.buttons,
            new_lights = bxor(lights, button),
            not MapSet.member?(visited, new_lights) do
          {new_lights, depth + 1, [button | path]}
        end

      case Enum.find(new_states, &(elem(&1, 0) == machine.indicators)) do
        {_, depth, path} ->
          {depth, Enum.reverse(path)}

        nil ->
          new_visited =
            for {lights, _, _} <- new_states, into: visited, do: lights

          bfs(queue ++ new_states, new_visited, machine)
      end
    end

    def configure_joltage(machine) do
      bfs2([{machine.current_joltage, 0, []}], MapSet.new(), machine)
    end

    defp bfs2([], _visited, _machine), do: :no_viable_sequence

    defp bfs2([{joltage, depth, path} | queue], visited, machine) do
      new_states =
        for button <- machine.buttons,
            Enum.all?(button, fn i ->
              Map.has_key?(machine.joltage_requirements, i)
            end),
            new_joltage =
              Enum.reduce(button, joltage, fn i, acc ->
                Map.update(acc, i, 1, &(&1 + 1))
              end),
            Enum.all?(new_joltage, fn {k, v} ->
              v <= Map.get(machine.joltage_requirements, k)
            end),
            not MapSet.member?(visited, new_joltage),
            do: {new_joltage, depth + 1, [button | path]}

      new_visited =
        for {joltage, _, _} <- new_states, into: visited, do: joltage

      case Enum.find(new_states, fn {joltage, _, _} ->
             joltage == machine.joltage_requirements
           end) do
        {_, depth, path} ->
          {depth, Enum.reverse(path)}

        nil ->
          bfs2(queue ++ new_states, new_visited, machine)
      end
    end
  end

  def part1(input) do
    Machine.from_string(input)
    |> Enum.map(&Machine.shortest_button_sequence/1)
    |> Enum.sum_by(fn {depth, _path} -> depth end)
  end

  def part2(input) do
    Machine.from_string(input)
    |> Task.async_stream(
      fn machine ->
        Machine.configure_joltage(machine)
      end,
      max_concurrency: System.schedulers_online(),
      ordered: false,
      timeout: :infinity
    )
    |> Enum.sum_by(fn {:ok, {depth, _path}} -> depth end)
  end
end
