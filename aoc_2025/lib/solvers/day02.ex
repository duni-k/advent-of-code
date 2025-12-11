defmodule AOCSolvers.Day02 do
  def part1(input) do
    solve(input, &repeated?(&1, 2))
  end

  def part2(input) do
    solve(input, &repeated?/1)
  end

  defp solve(input, is_invalid_id) do
    input
    |> String.split(",")
    |> Task.async_stream(
      fn range ->
        range
        |> String.split("-")
        |> Enum.map(&String.to_integer/1)
        |> then(fn [first, second] -> first..second end)
        |> Enum.sum_by(&if is_invalid_id.(Integer.digits(&1)), do: &1, else: 0)
      end,
      ordered: false
    )
    |> Enum.sum_by(fn {:ok, sum} -> sum end)
  end

  defp repeated?(list), do: repeated?(list, length(list))

  defp repeated?(list, _) when length(list) < 2, do: false

  defp repeated?(list, n) do
    2..n
    |> Enum.filter(&(rem(n, &1) == 0))
    |> Enum.any?(fn n ->
      [first_chunk | rest_chunks] = Enum.chunk_every(list, div(length(list), n))
      Enum.all?(rest_chunks, &(&1 == first_chunk))
    end)
  end
end
