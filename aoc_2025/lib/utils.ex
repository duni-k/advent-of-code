defmodule AOC.Point do
  alias AOC.Point, as: Point

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
end
