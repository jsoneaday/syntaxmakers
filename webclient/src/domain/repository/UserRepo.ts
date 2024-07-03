import { OutputBool, USER_CHANGE_PASSWORD_URL } from "./Api";
import { DevOrEmployer } from "./AuthRepo";

export async function changePassword(
  devId: number,
  oldPassword: string,
  newPassword: string,
  devOrEmp: DevOrEmployer,
  access_token: string
) {
  const response = await fetch(USER_CHANGE_PASSWORD_URL, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${access_token}`,
    },
    body: JSON.stringify({
      id: devId,
      oldPassword,
      newPassword,
      devOrEmp,
    }),
  });

  if (response.ok) {
    const changePassResult: OutputBool = await response.json();
    return changePassResult.result;
  }
  return false;
}
