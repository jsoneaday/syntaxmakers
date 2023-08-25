import { Key } from "react";
import { Industry } from "../../domain/repository/IndustryRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

type IndustryModel = {
  key: Key;
  id: string;
  updatedAt: string;
  name: string;
};
export default IndustryModel;

export function convert(industry: Industry): IndustryModel {
  return {
    key: uuidv4(),
    id: industry.id,
    updatedAt: industry.updatedAt,
    name: industry.name,
  };
}
