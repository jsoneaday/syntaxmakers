import { LOGIN_URL, REFRESH_TOKEN_URL } from "./Api";

export enum DevOrEmployer {
  Developer = "Developer",
  Employer = "Employer",
}

export type LoginResult = {
  message: string;
  status: number;
};

export async function login(
  devOrEmp: DevOrEmployer,
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
        devOrEmp,
        email,
        password,
      }),
    });

    if (response.ok) {
      const access_token: string = await response.text();
      return {
        message: access_token,
        status: response.status,
      };
    }
    return {
      message: response.statusText,
      status: response.status,
    };
  } catch (err) {
    console.log("login failed", err);
  }

  return {
    message: "",
    status: 400,
  };
}

export async function refreshAccessToken(
  oldToken: string,
  devOrEmp: DevOrEmployer
) {
  try {
    const response = await fetch(REFRESH_TOKEN_URL, {
      method: "post",
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        oldToken,
        devOrEmp,
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
