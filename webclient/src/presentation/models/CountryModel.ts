import { Key } from "react";
import { Country } from "../../domain/repository/CountryRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

type CountryModel = {
  key: Key;
  id: string;
  updatedAt: string;
  name: string;
};
export default CountryModel;

export function convert(country: Country): CountryModel {
  return {
    key: uuidv4(),
    id: country.id,
    updatedAt: country.updatedAt,
    name: country.name,
  };
}
