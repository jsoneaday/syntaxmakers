import { Key } from "react";
import { KeyItem } from "../common/utils";

export default class JobPost implements KeyItem {
  constructor(
    public key: Key,
    public id: bigint,
    public updatedAt: string,
    public title: string,
    public description: string,
    public companyName: string,
    public isRemote: boolean,
    public primaryLangName: string,
    public secondaryLangName: string,
    public industryName: String,
    public salary: string,
    public logo?: Blob,
    public countryName?: string
  ) {}
}
