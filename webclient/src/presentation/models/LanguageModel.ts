import { Key } from "react";
import { Language } from "../../domain/repository/LanguageRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

type LanguageModel = {
  key: Key;
  id: string;
  updatedAt: string;
  name: string;
};
export default LanguageModel;

export function convert(language: Language): LanguageModel {
  return {
    key: uuidv4(),
    id: language.id,
    updatedAt: language.updatedAt,
    name: language.name,
  };
}
