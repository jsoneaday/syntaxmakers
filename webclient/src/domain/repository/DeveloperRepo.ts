import {
  DEVELOPER_EMAIL_URL,
  DEVELOPER_UPDATE_URL,
  DEVELOPER_URL,
  OutputBool,
} from "./Api";

export class Developer {
  constructor(
    public id: string, // bigint is not serializable by Redux
    public updatedAt: string, // iso string
    public userName: string,
    public fullName: string,
    public email: string,
    public description: string,
    public primaryLangId: number,
    public secondaryLangId?: number | null
  ) {}
}

export async function createDeveloper(newDev: {
  userName: string;
  fullName: string;
  email: string;
  description: string;
  password: string;
  primaryLangId: number;
  secondaryLangId?: number | null;
}) {
  const response = await fetch(DEVELOPER_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(newDev),
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }
  if (response.ok) {
    const newDev: { id: number } = await response.json();
    return newDev.id;
  } else {
    throw new Error(await response.text());
  }
}

export async function updateDeveloper(updateDev: {
  id: number; // dev id
  fullName: string;
  email: string;
  description: string;
  primaryLangId: number;
  secondaryLangId?: number | null;
  access_token: string;
}) {
  const response = await fetch(DEVELOPER_UPDATE_URL, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${updateDev.access_token}`,
    },
    body: JSON.stringify(updateDev),
  });

  if (!response.ok) {
    throw new Error(await response.text());
  }
  if (response.ok) {
    const changePassResult: OutputBool = await response.json();
    return changePassResult.result;
  }
  return false;
}

export async function getDeveloper(id: number) {
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
