import { INDUSTRIES_URL } from "./Api";

export class Industry {
  constructor(
    public id: string,
    public updatedAt: string,
    public name: string
  ) {}
}

export async function getIndustries(): Promise<Industry[]> {
  const response = await fetch(INDUSTRIES_URL, {
    method: "get",
  });

  if (response.ok) {
    return await response.json();
  }
  return [];
}
