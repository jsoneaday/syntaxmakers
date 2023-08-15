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
  try {
    const response = await fetch(LOGIN_URL, {
      method: "post",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        isDevOrEmp: isDevOrEmployer,
        email,
        password,
      }),
    });

    if (response.ok) {
      const access_token: string = await response.text();
      return access_token;
    }
  } catch (err) {
    console.log("login failed", err);
  }

  return "";
}
