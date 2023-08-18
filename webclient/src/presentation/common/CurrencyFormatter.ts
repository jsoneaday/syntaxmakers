export const currencyFormatter = new Intl.NumberFormat("en-US", {
  style: "currency",
  currency: "USD",
  minimumFractionDigits: 0,
});

export function appendPlusLargeCurrency(salary: string) {
  const jobSalary = Number(salary.replace(/,|\$/g, ""));
  if (jobSalary >= 400000) {
    return currencyFormatter.format(jobSalary) + "+";
  } else {
    return currencyFormatter.format(jobSalary);
  }
}
