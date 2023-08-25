import { Key } from "react";
import { Company } from "../../domain/repository/CompanyRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

type CompanyModel = {
  key: Key;
  id: string;
  updatedAt: string;
  name: string;
};
export default CompanyModel;

export function convert(company: Company): CompanyModel {
  return {
    key: uuidv4(),
    id: company.id,
    updatedAt: company.updatedAt,
    name: company.name,
  };
}
