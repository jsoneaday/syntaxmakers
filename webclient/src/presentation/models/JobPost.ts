import { Key } from "react";
import { KeyItem } from "./ModelUtils";
import { Job } from "../../domain/repository/JobRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { formatDistanceToNow, parseISO } from "date-fns";
import { currencyFormatter } from "../common/CurrencyFormatter";
import { shortenFormattedDateStr } from "../../domain/dateUtils";

export default class JobPost implements KeyItem {
  constructor(
    public key: Key,
    public id: string,
    public updatedAt: string,
    public title: string,
    public description: string,
    public employerId: string,
    public employerName: string,
    public companyId: string,
    public companyName: string,
    public isRemote: boolean,
    public primaryLangId: string,
    public primaryLangName: string,
    public secondaryLangId: string,
    public secondaryLangName: string,
    public industryId: String,
    public industryName: String,
    public salaryId: string,
    public salary: string,
    public companyLogo?: Blob,
    public countryId?: string,
    public countryName?: string
  ) {}
}

export function convert(job: Job) {
  const updatedAt = shortenFormattedDateStr(
    formatDistanceToNow(parseISO(job.updatedAt), {
      addSuffix: true,
    })
  );
  const companyLogoUInt8Array = job.companyLogo
    ? new Uint8Array(job.companyLogo)
    : undefined;

  return new JobPost(
    uuidv4(),
    job.id,
    updatedAt,
    job.title,
    job.description,
    job.employerId,
    job.employerName,
    job.companyId,
    job.companyName,
    job.isRemote,
    job.primaryLangId,
    job.primaryLangName,
    job.secondaryLangId,
    job.secondaryLangName,
    job.industryId,
    job.industryName,
    job.salaryId,
    currencyFormatter.format(Number(job.salary)),
    companyLogoUInt8Array ? new Blob([companyLogoUInt8Array]) : undefined,
    job.countryId,
    job.countryName
  );
}
