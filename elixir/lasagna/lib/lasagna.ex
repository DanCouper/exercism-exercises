defmodule Lasagna do
	@spec expected_minutes_in_oven() :: non_neg_integer()
	def expected_minutes_in_oven() do
		40
	end

	@spec remaining_minutes_in_oven(non_neg_integer()) :: non_neg_integer()
	def remaining_minutes_in_oven(elapsed_time) do
		expected = expected_minutes_in_oven()

		if elapsed_time > expected do
			0
		else
			expected - elapsed_time
		end
	end

	@spec preparation_time_in_minutes(non_neg_integer()):: non_neg_integer()
	def preparation_time_in_minutes(layers) do
		layers * 2
	end

	@spec total_time_in_minutes(non_neg_integer(), non_neg_integer()):: non_neg_integer()
	def total_time_in_minutes(layers, elapsed_time) do
		preparation_time_in_minutes(layers) + elapsed_time
	end

	@spec alarm() :: String.t()
	def alarm() do
	  "Ding!"
	end
end
