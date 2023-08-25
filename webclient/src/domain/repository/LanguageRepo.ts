import { LANGS_URL } from "./Api";

export class Language {
  constructor(
    public id: string,
    public updatedAt: string,
    public name: string
  ) {}
}

export async function getLanguages(): Promise<Language[]> {
  const response = await fetch(LANGS_URL, {
    method: "get",
  });

  if (response.ok) {
    return await response.json();
  }
  return [];
}
