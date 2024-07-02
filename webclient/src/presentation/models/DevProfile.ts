import { Key } from "react";
import { Developer } from "../../domain/repository/DeveloperRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

type DevProfile = {
  key: Key;
  id: string; // bigint is not serializable by Redux
  updatedAt: string;
  userName: string;
  fullName: string;
  email: string;
  primaryLangId: number;
  secondaryLangId?: number | null;
  accessToken?: string | null;
};
export default DevProfile;

export function convert(dev: Developer, accessToken?: string): DevProfile {
  return {
    key: uuidv4(),
    id: dev.id, // bigint is not serializable by Redux
    updatedAt: dev.updatedAt,
    userName: dev.userName,
    fullName: dev.fullName,
    email: dev.email,
    primaryLangId: dev.primaryLangId,
    secondaryLangId: dev.secondaryLangId,
    accessToken,
  };
}
