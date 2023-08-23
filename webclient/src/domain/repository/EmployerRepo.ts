import { EMPLOYER_EMAIL_URL } from "./Api";

export class Employer {
  constructor(
    public id: string, // bigint is not serializable by Redux
    public updatedAt: string, // iso string
    public userName: string,
    public fullName: string,
    public email: string,
    public company_id: string
  ) {}
}

export async function getEmployerByEmail(email: string, access_token: string) {
  const response = await fetch(`${EMPLOYER_EMAIL_URL}/${email}`, {
    method: "get",
    credentials: "include",
    headers: {
      Authorization: `Bearer ${access_token}`,
    },
  });

  if (response.ok) {
    const emp: Employer = await response.json();
    return emp;
  }
  return null;
}
