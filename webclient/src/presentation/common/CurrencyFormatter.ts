export const currencyFormatter = new Intl.NumberFormat("en-US", {
  style: "currency",
  currency: "USD",
  minimumFractionDigits: 0,
});

export function appendPlusLargeCurrency(salary: string) {
  const jobSalary = salary.replace
    ? Number(salary.replace(/,|\$/g, ""))
    : Number(salary);
  if (jobSalary >= 500000) {
    return `${currencyFormatter.format(jobSalary)}+`;
  } else {
    return currencyFormatter.format(jobSalary);
  }
}
