defmodule AOC.Solvers.Day06 do
  def part1(input) do
    [operators | operand_rows] =
      String.split(input, "\n") |> Enum.reverse()

    operations =
      Enum.map(operand_rows, fn line ->
        String.split(line)
      end)
      |> Enum.zip_with(& &1)
      |> Enum.map(fn line ->
        Enum.reverse(line) |> Enum.map(&String.to_integer/1)
      end)
      |> Enum.zip(String.split(operators))

    Enum.sum_by(operations, fn {operands, operator} ->
      evaluate(operator, operands)
    end)
  end

  def part2(input) do
    [operators | operand_rows] =
      String.trim_trailing(input) |> String.split("\n") |> Enum.reverse()

    operators = String.split(operators)
    max_len = Enum.map(operand_rows, &String.length/1) |> Enum.max()

    operations =
      Enum.reverse(operand_rows)
      |> Enum.map(
        &(String.pad_trailing(&1, max_len, " ")
          |> String.graphemes())
      )
      |> Enum.zip_with(& &1)
      |> Enum.chunk_by(&all_spaces?/1)
      |> Enum.reject(fn group ->
        Enum.all?(hd(group), &(&1 == " "))
      end)
      |> Enum.map(fn sub_matrix ->
        Enum.map(sub_matrix, fn row ->
          Enum.join(row)
          |> String.trim()
          |> String.to_integer()
        end)
      end)
      |> Enum.zip(operators)

    Enum.sum_by(operations, fn {operands, operator} ->
      evaluate(operator, operands)
    end)
  end

  defp evaluate(operator, operands) do
    case operator do
      "+" -> Enum.sum(operands)
      "*" -> Enum.reduce(operands, 1, &Kernel.*/2)
    end
  end

  defp all_spaces?(col), do: Enum.all?(col, &(&1 == " "))
end
