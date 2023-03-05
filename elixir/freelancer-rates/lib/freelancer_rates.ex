defmodule FreelancerRates do
  @daily_rate_multiplier 8.0
	@monthly_billable_hours 22.0

	@spec daily_rate(non_neg_integer()):: float()
  def daily_rate(hourly_rate) do
		hourly_rate * @daily_rate_multiplier
  end

  @spec apply_discount(float() | non_neg_integer(), float() | non_neg_integer()) :: float()
  def apply_discount(before_discount, discount) do
		to_subtract = (before_discount / 100) * discount
		before_discount - to_subtract
  end

	@spec monthly_rate(non_neg_integer(), float()) :: non_neg_integer()
  def monthly_rate(hourly_rate, discount) do
		hourly_rate
		|> FreelancerRates.daily_rate
		|> Kernel.*(@monthly_billable_hours)
		|> FreelancerRates.apply_discount(discount)
		|> Float.ceil(0)
  end

  def days_in_budget(budget, hourly_rate, discount) do
    with cost_per_month <- monthly_rate(hourly_rate, discount),
				 cost_per_day <- daily_rate(hourly_rate),
				 months <- div(budget, cost_per_month),
				 remaining_budget <- rem(budget, cost_per_month),
				 days <- div(remaining_budget, cost_per_day),
				 do: days + months * @monthly_billable_hours
  end
end
