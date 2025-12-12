defmodule AOCSolvers.Day12 do
  defmodule Shape do
    defstruct [:width, :height, :coords]

    def new(width, height, coords) do
      %Shape{width: width, height: height, coords: coords}
    end

    def rotate(%Shape{width: width, height: height, coords: coords}) do
      rotated = MapSet.new(coords, fn {x, y} -> {width - y - 1, x} end)
      %Shape{width: height, height: width, coords: rotated}
    end

    def flip(%Shape{width: width, height: height, coords: coords}) do
      flipped = MapSet.new(coords, fn {x, y} -> {width - x - 1, y} end)
      %Shape{width: width, height: height, coords: flipped}
    end

    def all_transformations(shape) do
      rotations = [
        shape,
        rotate(shape),
        shape |> rotate() |> rotate(),
        shape |> rotate() |> rotate() |> rotate()
      ]

      (rotations ++ Enum.map(rotations, &flip/1))
      |> Enum.uniq_by(& &1.coords)
    end
  end

  def part1(input) do
    {shapes, regions} = parse(input)

    Enum.count(regions, fn region ->
      shapes_to_place =
        region.quantities
        |> Enum.flat_map(fn {idx, quantity} ->
          List.duplicate(shapes[idx], quantity)
        end)
        |> Enum.sort_by(&MapSet.size(&1.coords), :desc)

      fits_in_region?(shapes_to_place, region)
    end)
  end

  defp fits_in_region?([], _region), do: true

  defp fits_in_region?(shapes, region) do
    total_area_needed = Enum.sum_by(shapes, &MapSet.size(&1.coords))
    available_area = region.width * region.height - MapSet.size(region.coords)

    total_area_needed <= available_area and
      fits_with_backtracking?(shapes, region)
  end

  defp fits_with_backtracking?([shape | rest], region) do
    Enum.any?(Shape.all_transformations(shape), fn transformed ->
      try_all_positions(transformed, rest, region)
    end)
  end

  defp try_all_positions(shape, remaining_shapes, region) do
    max_x = region.width - shape.width
    max_y = region.height - shape.height

    max_x >= 0 and max_y >= 0 and
      Enum.any?(0..max_x, fn offset_x ->
        Enum.any?(0..max_y, fn offset_y ->
          place_shape_at(shape, offset_x, offset_y, remaining_shapes, region)
        end)
      end)
  end

  defp place_shape_at(shape, offset_x, offset_y, remaining_shapes, region) do
    translated = translate_coords(shape.coords, offset_x, offset_y)

    if MapSet.disjoint?(translated, region.coords) do
      updated_region = %{
        region
        | coords: MapSet.union(region.coords, translated)
      }

      fits_in_region?(remaining_shapes, updated_region)
    else
      false
    end
  end

  defp translate_coords(coords, offset_x, offset_y) do
    MapSet.new(coords, fn {x, y} -> {x + offset_x, y + offset_y} end)
  end

  defp parse(input) do
    [regions_block | shape_blocks] =
      input |> String.split("\n\n", trim: true) |> Enum.reverse()

    shapes = parse_shapes(shape_blocks)
    regions = parse_regions(regions_block)

    {shapes, regions}
  end

  defp parse_shapes(shape_blocks) do
    Enum.map(shape_blocks, fn block ->
      [idx_line | rows] = String.split(block, "\n", trim: true)
      idx = idx_line |> String.trim_trailing(":") |> String.to_integer()

      width = Enum.max_by(rows, &String.length/1) |> String.length()
      height = length(rows)

      coords =
        for {row, y} <- Enum.with_index(rows),
            {ch, x} <- row |> String.graphemes() |> Enum.with_index(),
            ch == "#",
            into: MapSet.new(),
            do: {x, y}

      {idx, Shape.new(width, height, coords)}
    end)
    |> Map.new()
  end

  defp parse_regions(regions_block) do
    regions_block
    |> String.split("\n", trim: true)
    |> Enum.map(fn line ->
      [size, quantities] = String.split(line, ":", trim: true)

      [width, height] =
        size |> String.split("x", trim: true) |> Enum.map(&String.to_integer/1)

      quantities =
        quantities
        |> String.split()
        |> Enum.map(&String.to_integer/1)
        |> Enum.with_index()
        |> Enum.reject(fn {quantity, _} -> quantity == 0 end)
        |> Map.new(fn {quantity, index} -> {index, quantity} end)

      %{
        width: width,
        height: height,
        quantities: quantities,
        coords: MapSet.new()
      }
    end)
  end
end
