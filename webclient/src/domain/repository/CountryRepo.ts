import { COUNTRIES_URL } from "./Api";

export class Country {
  constructor(
    public id: string,
    public updatedAt: string,
    public name: string
  ) {}
}

export async function getCountries(): Promise<Country[]> {
  const response = await fetch(COUNTRIES_URL, {
    method: "get",
  });

  if (response.ok) {
    return await response.json();
  }
  return [];
}
