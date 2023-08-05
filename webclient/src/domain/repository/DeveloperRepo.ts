import { DEVELOPER_URL } from "./Api";

export class Developer {
  constructor(
    public id: bigint,
    public updated_at: Date,
    public user_name: string,
    public full_name: string,
    public email: string,
    public primary_lang_id: bigint,
    public secondary_lang_id?: bigint
  ) {}
}

export async function getDeveloper(id: bigint) {
  const response = await fetch(`${DEVELOPER_URL}/${id}`, {
    method: "get",
  });

  if (response.ok) {
    const developerObj: Developer = await response.json();
    return developerObj;
  }
  return null;
}
