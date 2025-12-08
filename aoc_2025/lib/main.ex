defmodule AOC do
  @moduledoc """
  Running solvers for Advent of Code.
  """
  def solve, do: solve(Date.utc_today().day)

  def solve(day) when is_integer(day), do: solve(day, 1)

  def solve(day, part) when is_integer(day) and is_integer(part) do
    case fetch_or_read_input(day) do
      {:ok, input} ->
        try do
          start = Time.utc_now()
          solution = solve!(day, part, input)
          IO.puts("Solution for problem #{pad(day)}#{pad(part)}: #{solution}")

          IO.puts(
            "Elapsed time: #{Time.diff(Time.utc_now(), start, :millisecond)}ms"
          )
        rescue
          UndefinedFunctionError ->
            IO.puts("Solver for #{pad(day)}#{pad(part)} not implemented.")
        end

      {:error, reason} ->
        IO.puts("Failed fetching input: #{reason}")
    end
  end

  defp solve!(day, part, input) do
    module_name = Module.concat([AOC.Solvers, "Day#{pad(day)}"])
    function_name = String.to_atom("part#{part}")

    apply(module_name, function_name, [input])
  end

  defp fetch_or_read_input(day) do
    input_file = "inputs/day#{pad(day)}.txt"

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
