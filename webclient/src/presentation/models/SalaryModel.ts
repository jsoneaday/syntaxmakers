import { Key } from "react";
import { Salary } from "../../domain/repository/SalaryRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

type SalaryModel = {
  key: Key;
  id: string;
  updatedAt: string;
  base: string;
};
export default SalaryModel;

export function convert(salary: Salary): SalaryModel {
  return {
    key: uuidv4(),
    id: salary.id,
    updatedAt: salary.updatedAt,
    base: salary.base,
  };
}
