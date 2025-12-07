defmodule AOC.Solvers.Day07 do
  defmodule Beam do
    defstruct position: 0, timelines: 1

    def split(%Beam{position: pos, timelines: timelines}) do
      [
        %Beam{position: pos - 1, timelines: timelines},
        %Beam{position: pos + 1, timelines: timelines}
      ]
    end

    def merge([%Beam{position: pos} | _] = beams) do
      %Beam{
        position: pos,
        timelines: Enum.sum_by(beams, & &1.timelines)
      }
    end
  end

  def part1(input) do
    {starting_beam, splitters_lines} = parse(input)

    initial_state = {MapSet.new([starting_beam]), 0}

    {_final_beams, total_splits} =
      Enum.reduce(splitters_lines, initial_state, &process_line/2)

    total_splits
  end

  defp process_line(splitters, {beams, split_count}) do
    for beam <- beams, reduce: {MapSet.new(), split_count} do
      {acc_beams, splits_so_far} ->
        if beam in splitters do
          {acc_beams |> MapSet.put(beam - 1) |> MapSet.put(beam + 1),
           splits_so_far + 1}
        else
          {MapSet.put(acc_beams, beam), splits_so_far}
        end
    end
  end

  def part2(input) do
    {first_beam, splitters_lines} = parse(input)

    first_beam = %Beam{position: first_beam}

    beams =
      Enum.reduce(splitters_lines, [first_beam], fn splitters, beams ->
        Enum.flat_map(beams, fn b ->
          if b.position in splitters, do: Beam.split(b), else: [b]
        end)
        |> Enum.group_by(& &1.position)
        |> Enum.map(fn {_, beams} -> Beam.merge(beams) end)
      end)

    Enum.sum_by(beams, & &1.timelines)
  end

  defp parse(input) do
    [start | rest] = String.split(input, "\n")

    splitter_lines =
      for line <- rest,
          not all_dots?(line),
          do: find_splitters(line)

    first_beam =
      start
      |> String.graphemes()
      |> Enum.find_index(&(&1 == "S"))

    {first_beam, splitter_lines}
  end

  defp find_splitters(line) do
    for {ch, i} <- String.graphemes(line) |> Enum.with_index(),
        ch == "^",
        into: MapSet.new(),
        do: i
  end

  defp all_dots?(line), do: String.trim(line, ".") === ""
end
