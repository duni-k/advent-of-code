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

    %{area: area} =
      for {c1, i} <- Enum.with_index(reds),
          {c2, j} <- Enum.with_index(reds),
          i < j do
        %{points: {c1, c2}, area: XY.rectangle_area(c1, c2)}
      end
      |> IO.inspect()
      |> Enum.sort_by(& &1.area, :desc)
      |> Enum.find(&all_green_or_red?(reds, &1.points))

    area
  end

  defp all_green_or_red?(reds, {%XY{x: x1, y: y1}, %XY{x: x2, y: y2}}) do
    # vi konstruerar de korresponderande hörnpunkterna och för varje hörn
    # verifierar vi att det finns en punkt i reds som ligger utom den
    # MEN VAD HÄNDER OM där 2 av q1-4 är i reds och därmed kandidater???
    #              XXXX#....#XXX
    #              X.q1-----q2.X
    #              X..| ... |..X
    #              X..|.....|..X
    #              X.q4-----q3.X
    #              #XXXXXXXXXX##
    {min_x, max_x} = {min(x1, x2), max(x1, x2)}
    {min_y, max_y} = {min(y1, y2), max(y1, y2)}
    q1 = XY.new(min_x, max_y)
    q2 = XY.new(max_x, max_y)
    q3 = XY.new(max_x, min_y)
    q4 = XY.new(min_x, min_y)

    top_edge =
      check_edge(Range.new(q1.x + 1, q2.x - 1), :x, q1.y, reds, &top?/2)

    right_edge =
      check_edge(Range.new(q3.y + 1, q2.y - 1), :y, q2.x, reds, &right?/2)

    bottom_edge =
      check_edge(Range.new(q4.x + 1, q3.x - 1), :x, q3.y, reds, &bottom?/2)

    left_edge =
      check_edge(Range.new(q4.y + 1, q1.y - 1), :y, q4.x, reds, &left?/2)

    top_edge and right_edge and bottom_edge and left_edge and
      Enum.any?(reds, &q1(q1, &1)) and Enum.any?(reds, &q2(q2, &1)) and
      Enum.any?(reds, &q3(q3, &1)) and Enum.any?(reds, &q4(q4, &1))
  end

  defp check_edge(range, coord_type, position, reds, check_fn) do
    Enum.reduce_while(range, false, fn coord, acc ->
      point =
        case coord_type do
          :x -> %XY{x: coord, y: position}
          :y -> %XY{x: position, y: coord}
        end

      if Enum.any?(reds, &check_fn.(point, &1)) do
        {:halt, true}
      else
        {:cont, acc}
      end
    end)
  end

  defp q1(q, p), do: top?(q, p) and (left?(q, p) or right?(q, p))
  defp q2(q, p), do: top?(q, p) and (left?(q, p) or right?(q, p))
  defp q3(q, p), do: bottom?(q, p) and (left?(q, p) or right?(q, p))
  defp q4(q, p), do: bottom?(q, p) and (left?(q, p) or right?(q, p))
  defp left?(q, p), do: p.x <= q.x
  defp right?(q, p), do: p.x >= q.x
  defp top?(q, p), do: p.y >= q.y
  defp bottom?(q, p), do: p.y <= q.y
end
