defmodule AOCSolvers.Day09 do
  alias AOCPoint, as: XY

  def part1(input) do
    candidates = XY.from_lines(input)

    for {c1, i} <- Enum.with_index(candidates),
        {c2, j} <- Enum.with_index(candidates),
        i < j,
        reduce: 0 do
      max_area ->
        max(max_area, XY.rectangle_area(c1, c2))
    end
  end

  def part2(input) do
    [first | rest] = XY.from_lines(input)

    reds =
      [first | Enum.reverse(rest)]
      |> Enum.reverse()

    greens =
      Enum.chunk_every([first | reds], 2, 1, :discard)
      |> Enum.flat_map(fn [c1, c2] ->
        populate_greens(c1, c2)
      end)
      |> MapSet.new()

    IO.inspect(MapSet.size(greens))

    %{area: area} =
      for {c1, i} <- Enum.with_index(reds),
          {c2, j} <- Enum.with_index(reds),
          i < j do
        %{points: {c1, c2}, area: XY.rectangle_area(c1, c2)}
      end
      |> Enum.sort_by(& &1.area, :desc)
      |> Enum.find(&all_green_or_red?(greens, &1.points))

    area
  end

  defp populate_greens(c1, c2) do
    if c1.x == c2.x do
      for y <- Range.new(min(c1.y, c2.y), max(c1.y, c2.y)) do
        XY.new(c1.x, y)
      end
    else
      for x <- Range.new(min(c1.x, c2.x), max(c1.x, c2.x)) do
        XY.new(x, c1.y)
      end
    end
  end

  defp all_green_or_red?(tiles, {%XY{x: x1, y: y1}, %XY{x: x2, y: y2}}) do
    {min_x, max_x} = {min(x1, x2), max(x1, x2)}
    {min_y, max_y} = {min(y1, y2), max(y1, y2)}

    chunk_size =
      div(MapSet.size(tiles), System.schedulers_online()) |> max(100)

    IO.inspect(chunk_size)

    not (tiles
         |> MapSet.to_list()
         |> Enum.chunk_every(chunk_size)
         |> Task.async_stream(
           fn chunk ->
             Enum.any?(chunk, fn %XY{x: x, y: y} ->
               x > min_x and x < max_x and y > min_y and y < max_y
             end)
           end,
           max_concurrency: System.schedulers_online(),
           ordered: false
         )
         |> Enum.any?(fn {:ok, has_inside?} -> has_inside? end))
    |> IO.inspect()
  end
end
