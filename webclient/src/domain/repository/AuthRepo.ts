import { LOGIN_URL } from "./Api";

export enum DevOrEmployer {
  Developer = "Developer",
  Employer = "Employer",
}

export async function login(
  isDevOrEmployer: DevOrEmployer,
  email: string,
  password: string
) {
  console.log("fetch", fetch);
  try {
    const result = await fetch(LOGIN_URL, {
      method: "post",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        isDevOrEmp: isDevOrEmployer,
        email,
        password,
      }),
    });
    if (result.ok) {
      const id: bigint = await result.json();
      return id;
    }
  } catch (err) {
    console.log("login failed", err);
  }

  return BigInt(0);
}
