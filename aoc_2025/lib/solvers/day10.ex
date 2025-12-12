defmodule AOCSolvers.Day10 do
  defmodule Machine do
    import Bitwise

    defstruct [:buttons, :indicators, lights: 0]

    def from_string(input) do
      for line <- String.split(input, "\n", trim: true) do
        [raw_light | raw_buttons] = String.split(line)
        [_ | raw_buttons] = Enum.reverse(raw_buttons)

        buttons =
          for raw_button <- Enum.reverse(raw_buttons),
              do:
                String.trim_leading(raw_button, "(")
                |> String.trim_trailing(")")
                |> String.split(",")
                |> Enum.map(&String.to_integer/1)
                |> Enum.reduce(0, fn n, acc -> acc ||| 1 <<< n end)

        indicators =
          String.trim_leading(raw_light, "[")
          |> String.trim_trailing("]")
          |> String.to_charlist()
          |> Enum.with_index()
          |> Enum.reduce(0, fn {ch, idx}, acc ->
            if ch == ?#, do: acc ||| 1 <<< idx, else: acc
          end)

        %Machine{indicators: indicators, buttons: buttons}
      end
    end

    def shortest_button_sequence(machine) do
      bfs([{machine.lights, 0, []}], MapSet.new(), machine)
    end

    defp bfs([], _visited, _machine), do: nil

    defp bfs([{lights, depth, path} | queue], visited, machine) do
      IO.inspect(path)
      IO.inspect(machine)

      new_states =
        for button <- machine.buttons,
            new_lights = bxor(lights, button),
            not MapSet.member?(visited, new_lights) do
          {new_lights, depth + 1, [button | path]}
        end

      case Enum.find(new_states, &(elem(&1, 0) == machine.indicators)) do
        {_, depth, path} ->
          {depth, Enum.reverse(path)} |> IO.inspect(label: "Found")

        nil ->
          new_visited =
            for {lights, _, _} <- new_states, into: visited, do: lights

          bfs(queue ++ new_states, new_visited, machine)
      end
    end
  end

  def part1(input) do
    Machine.from_string(input)
    |> IO.inspect()
    |> Enum.map(&Machine.shortest_button_sequence/1)
    |> Enum.sum_by(fn {depth, _path} -> depth end)
  end
end
