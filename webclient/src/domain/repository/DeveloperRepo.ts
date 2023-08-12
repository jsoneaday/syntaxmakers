import { DEVELOPER_URL } from "./Api";

export class Developer {
  constructor(
    public id: string, // bigint is not serializable by Redux
    public updated_at: Date,
    public user_name: string,
    public full_name: string,
    public email: string,
    public primary_lang_id: string,
    public secondary_lang_id?: string | null
  ) {}
}

export async function getDeveloper(id: string) {
  console.log(`${DEVELOPER_URL}/${id}`);
  const response = await fetch(`${DEVELOPER_URL}/${id}`, {
    method: "get",
  });

  if (response.ok) {
    const developerObj: Developer = await response.json();
    return developerObj;
  }
  return null;
}
