import {
  EMPLOYER_EMAIL_URL,
  EMPLOYER_UPDATE_URL,
  EMPLOYER_URL,
  OutputBool,
} from "./Api";

export class Employer {
  constructor(
    public id: string, // bigint is not serializable by Redux
    public updatedAt: string, // iso string
    public userName: string,
    public fullName: string,
    public email: string,
    public companyId: string
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

/// todo: if email is changed confirmation email must go out
/// When creating a new company, newCompanyName must have a value but companyId must be null
export async function createEmployer(newEmp: {
  userName: string;
  fullName: string;
  email: string;
  password: string;
  companyId: string | null;
  newCompanyName: string | null;
}) {
  const response = await fetch(EMPLOYER_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(newEmp),
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }
  if (response.ok) {
    const entity: { id: number } = await response.json();
    return entity.id;
  } else {
    throw new Error(await response.text());
  }
}

export async function updateEmployer(updateEmp: {
  id: number; // dev id
  fullName: string;
  email: string;
  companyId: string | null;
  newCompanyName: string | null;
  access_token: string;
}) {
  console.log("update emp", updateEmp);
  const response = await fetch(EMPLOYER_UPDATE_URL, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${updateEmp.access_token}`,
    },
    body: JSON.stringify(updateEmp),
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }
  if (response.ok) {
    const result: OutputBool = await response.json();
    return result.result;
  }
  return false;
}
