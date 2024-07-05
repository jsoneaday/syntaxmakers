import { Key } from "react";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { Employer } from "../../domain/repository/EmployerRepo";

type EmpProfile = {
  key: Key;
  id: string;
  updatedAt: string;
  userName: string;
  fullName: string;
  email: string;
  companyId: string;
  accessToken?: string | null;
};
export default EmpProfile;

export function convert(emp: Employer, accessToken?: string): EmpProfile {
  return {
    key: uuidv4(),
    id: emp.id,
    updatedAt: emp.updatedAt,
    userName: emp.userName,
    fullName: emp.fullName,
    email: emp.email,
    companyId: emp.companyId,
    accessToken,
  };
}
