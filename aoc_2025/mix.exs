defmodule AOC.MixProject do
  use Mix.Project

  def project do
    [
      app: :aoc,
      version: "0.20.25",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      escript: [main_module: AOCRunner]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger, :elixir]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:httpoison, "~> 2.3"}
    ]
  end
end
