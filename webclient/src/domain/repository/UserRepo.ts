import {
  OutputBool,
  RESET_PASSWORD_URL,
  SEND_EMAIL_URL,
  USER_CHANGE_PASSWORD_URL,
} from "./Api";
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

export async function sendEmail(
  empId: string,
  devId: string,
  subject: string,
  body: string,
  access_token: string
) {
  const response = await fetch(SEND_EMAIL_URL, {
    method: "POST",
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${access_token}`,
    },
    body: JSON.stringify({
      senderEmpId: empId,
      receiverDevId: devId,
      subject,
      body,
    }),
  });

  if (response.ok) {
    const changePassResult: OutputBool = await response.json();
    return changePassResult.result;
  }
  return false;
}

export async function resetPassword(
  userId: number,
  newPassword: string,
  devOrEmp: DevOrEmployer,
  uniqueKey: string
): Promise<OutputBool> {
  const response = await fetch(RESET_PASSWORD_URL, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      userId,
      newPassword,
      devOrEmp,
      uniqueKey,
    }),
  });

  if (!response.ok) {
    return {
      result: false,
      message: await response.text(),
    };
  }
  if (response.ok) {
    const resetPassResult: OutputBool = await response.json();
    return resetPassResult;
  }
  return {
    result: false,
    message: undefined,
  };
}
