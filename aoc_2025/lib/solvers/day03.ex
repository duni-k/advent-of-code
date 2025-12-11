defmodule AOCSolvers.Day03 do
  def part1(input) do
    solve(input, &best_joltage(&1, 2))
  end

  def part2(input) do
    solve(input, &best_joltage(&1, 12))
  end

  def best_joltage(bank, n_batteries) do
    len = length(bank)

    Enum.reduce(1..n_batteries, {0, 0}, fn current, {acc, start_pos} ->
      # b b b | _ _ _ _ | mb mb mb mb
      end_pos = len - (n_batteries - current + 1)

      {max_digit, max_idx} =
        bank
        |> Enum.slice(start_pos..end_pos)
        |> Enum.with_index(start_pos)
        |> Enum.max_by(fn {digit, _} -> digit end)

      multiplier = :math.pow(10, n_batteries - current) |> trunc()
      {acc + max_digit * multiplier, max_idx + 1}
    end)
    |> then(fn {sum, _} -> sum end)
  end

  defp solve(input, best_joltage) do
    input
    |> String.split("\n", trim: true)
    |> Enum.map(&(String.to_integer(&1) |> Integer.digits()))
    |> Task.async_stream(best_joltage, ordered: false)
    |> Enum.sum_by(fn {:ok, joltage} -> joltage end)
  end
end
