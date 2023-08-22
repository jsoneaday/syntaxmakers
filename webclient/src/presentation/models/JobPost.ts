import { Key } from "react";
import { KeyItem } from "./ModelUtils";
import { Job } from "../../domain/repository/JobRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { formatDistanceToNow, parseISO } from "date-fns";
import { currencyFormatter } from "../common/CurrencyFormatter";

export default class JobPost implements KeyItem {
  constructor(
    public key: Key,
    public id: string,
    public updatedAt: string,
    public title: string,
    public description: string,
    public employerName: string,
    public companyName: string,
    public isRemote: boolean,
    public primaryLangName: string,
    public secondaryLangName: string,
    public industryName: String,
    public salary: string,
    public companyLogo?: Blob,
    public countryName?: string
  ) {}
}

export function convert(job: Job) {
  const updatedAt = formatDistanceToNow(parseISO(job.updatedAt), {
    addSuffix: true,
  });
  const companyLogoUInt8Array = job.companyLogo
    ? new Uint8Array(job.companyLogo)
    : undefined;

  return new JobPost(
    uuidv4(),
    job.id,
    updatedAt,
    job.title,
    job.description,
    job.employerName,
    job.companyName,
    job.isRemote,
    job.primaryLangName,
    job.secondaryLangName,
    job.industryName,
    currencyFormatter.format(Number(job.salary)),
    companyLogoUInt8Array ? new Blob([companyLogoUInt8Array]) : undefined,
    job.countryName
  );
}
