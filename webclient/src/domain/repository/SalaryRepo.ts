import { SALARIES_URL } from "./Api";

export class Salary {
  constructor(
    public id: string,
    public updatedAt: string,
    public base: string
  ) {}
}

export async function getSalaries(): Promise<Salary[]> {
  const response = await fetch(SALARIES_URL, {
    method: "get",
  });

  if (response.ok) {
    return await response.json();
  }
  return [];
}
