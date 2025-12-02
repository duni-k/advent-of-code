defmodule AOC.Client do
  @moduledoc """
  A simple HTTP client for fetching content from the Advent of Code website
  using a session cookie.
  """

  @aoc_url "https://adventofcode.com/2025/day"

  @doc """
  Fetches the content of a given path on the Advent of Code website.
  """
  def fetch(day, session_cookie) do
    url = "#{@aoc_url}/#{day}/input"
    headers = [{"Cookie", "session=#{session_cookie}"}]

    case HTTPoison.get(url, headers) do
      {:ok, %HTTPoison.Response{status_code: 200, body: body}} ->
        {:ok, body |> String.trim()}

      {:ok, %HTTPoison.Response{status_code: status_code}} ->
        {:error, "HTTP request failed with status: #{status_code}"}

      {:error, %HTTPoison.Error{reason: reason}} ->
        {:error, "HTTP request failed: #{inspect(reason)}"}
    end
  end
end
