defmodule AOCRunner do
  @moduledoc """
  Running solvers for Advent of Code.
  """
  def solve, do: solve(Date.utc_today().day)

  def solve(day) when is_integer(day), do: solve(day, 1)

  def solve(day, part) when is_integer(day) and is_integer(part) do
    case fetch_or_read_input(day) do
      {:ok, input} ->
        start = Time.utc_now()
        solution = solve!(day, part, input)
        IO.puts("Solution for problem #{pad(day)}#{pad(part)}: #{solution}")

        IO.puts(
          "Elapsed time: #{Time.diff(Time.utc_now(), start, :millisecond)}ms"
        )

      {:error, reason} ->
        IO.puts("Failed fetching input: #{reason}")
    end
  end

  defp solve!(day, part, input) do
    module_name = Module.concat([AOCSolvers, "Day#{pad(day)}"])
    function_name = String.to_atom("part#{part}")

    apply(module_name, function_name, [input])
  end

  defp fetch_or_read_input(day) do
    # input_file = "inputs/day#{pad(day)}.txt"
    input_file = "inputs/test"

    cond do
      File.exists?(input_file) ->
        {:ok, File.read!(input_file)}

      is_nil(System.get_env("AOC_COOKIE")) ->
        {:error, "AOC_COOKIE environment variable is not set."}

      true ->
        case AOC.Client.fetch(day, System.get_env("AOC_COOKIE")) do
          {:ok, input} ->
            File.write!(input_file, input)
            {:ok, input}

          error ->
            error
        end
    end
  end

  defp pad(to_pad, len \\ 2) do
    String.pad_leading(to_string(to_pad), len, "0")
  end
end
