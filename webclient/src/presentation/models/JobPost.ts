import { Key } from "react";
import { KeyItem } from "../common/utils";
import { Job } from "../../domain/repository/JobRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { formatDistanceToNow } from "date-fns";

export default class JobPost implements KeyItem {
  constructor(
    public key: Key,
    public id: bigint,
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
    public logo?: Blob,
    public countryName?: string
  ) {}
}

export function convert(job: Job) {
  const updatedAt = formatDistanceToNow(job.updatedAt, {
    addSuffix: true,
  });

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
    job.salary,
    job.companyLogo,
    job.countryName
  );
}
