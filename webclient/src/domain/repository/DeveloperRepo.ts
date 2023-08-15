import { DEVELOPER_EMAIL_URL, DEVELOPER_URL } from "./Api";

export class Developer {
  constructor(
    public id: string, // bigint is not serializable by Redux
    public updatedAt: string, // iso string
    public userName: string,
    public fullName: string,
    public email: string,
    public primaryLangId: string,
    public secondaryLangId?: string | null
  ) {}
}

export async function getDeveloper(id: string) {
  const response = await fetch(`${DEVELOPER_URL}/${id}`, {
    method: "get",
  });

  if (response.ok) {
    const developerObj: Developer = await response.json();
    return developerObj;
  }
  return null;
}

export async function getDeveloperByEmail(email: string, access_token: string) {
  const response = await fetch(`${DEVELOPER_EMAIL_URL}/${email}`, {
    method: "get",
    credentials: "include",
    headers: {
      Authorization: `Bearer ${access_token}`,
    },
  });

  if (response.ok) {
    const developerObj: Developer = await response.json();
    return developerObj;
  }
  return null;
}
