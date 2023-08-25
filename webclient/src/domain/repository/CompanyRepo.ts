import { COMPANIES_URL } from "./Api";

export class Company {
  constructor(
    public id: string,
    public updatedAt: string,
    public name: string
  ) {}
}

export async function getCompanies(): Promise<Company[]> {
  const response = await fetch(COMPANIES_URL, {
    method: "get",
  });

  if (response.ok) {
    return await response.json();
  }
  return [];
}
