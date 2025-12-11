defmodule AOCPoint do
  alias AOCPoint, as: Point

  @enforce_keys [:x, :y]
  defstruct [:x, :y, :z]

  @type t :: %__MODULE__{
          x: integer(),
          y: integer(),
          z: integer() | nil
        }

  @spec from_lines(String.t(), col) :: col when col: Collectable.t()
  def from_lines(input, into \\ []) do
    for line <- String.split(input, "\n", trim: true),
        coord = String.split(line, ",") |> Enum.map(&String.to_integer/1),
        into,
        do: Point.new(coord)
  end

  @spec new([integer()]) :: Point.t()
  def new([x, y]), do: %Point{x: x, y: y}
  def new([x, y, z]), do: %Point{x: x, y: y, z: z}
  @spec new(integer(), integer()) :: Point.t()
  def new(x, y), do: %Point{x: x, y: y}
  @spec new(integer(), integer(), integer()) :: Point.t()
  def new(x, y, z), do: %Point{x: x, y: y, z: z}

  @spec rectangle_area(Point.t(), Point.t()) :: integer()
  def rectangle_area(c1, c2) do
    (abs(c2.x - c1.x) + 1) * (abs(c2.y - c1.y) + 1)
  end

  def print_grid(points) do
    max_width = Enum.max_by(points, & &1.x).x + 1
    max_height = Enum.max_by(points, & &1.y).y + 1

    for y <- 0..(max_height + 1) do
      for x <- 0..(max_width + 1) do
        if MapSet.member?(points, Point.new(x, y)), do: "#", else: "."
      end
      |> Enum.join("")
    end
    |> Enum.join("\n")
    |> IO.puts()
  end
end
