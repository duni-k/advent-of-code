defmodule AOC.Utils.InputParserTest do
  use ExUnit.Case
  alias AOC.InputParser

  describe "lines/1" do
    test "splits input into trimmed lines" do
      input = "line1\nline2\nline3\n"
      expected = ["line1", "line2", "line3"]

      assert InputParser.parse_lines(input) == expected
    end

    test "trims empty lines" do
      input = "line1\n\nline2\n\nline3\n"
      expected = ["line1", "line2", "line3"]

      assert InputParser.parse_lines(input) == expected
    end
  end

  describe "tokens/1" do
    test "splits lines into tokens separated by whitespace" do
      input = "a b c\nd e f\ng h i"
      expected = [["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]]

      assert InputParser.parse_tokens(input) == expected
    end

    test "handles multiple spaces between tokens" do
      input = "a   b c\n  d e   f\n g h i"
      expected = [["a", "b", "c"], ["d", "e", "f"], ["g", "h", "i"]]

      assert InputParser.parse_tokens(input) == expected
    end
  end
end
