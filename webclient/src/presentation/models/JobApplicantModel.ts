/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import { JobApplicant } from "../../domain/repository/JobRepo";
import { friendlyDate } from "../../domain/dateUtils";
import { GroupItem } from "./ModelUtils";

export class JobApplicantModel implements GroupItem {
  constructor(
    public key: string,
    /// job id
    public id: string,
    /// job title
    public title: string,
    public jobUpdatedAt: string,
    public appliedAt: string,
    public devId: string,
    public fullName: string,
    public description: string,
    public primaryLangId: string,
    public primaryLangName: string,
    public secondaryLangId: string,
    public secondaryLangName: string
  ) {}
}

export function convert(applicant: JobApplicant): JobApplicantModel {
  return {
    key: uuidv4(),
    id: applicant.jobId,
    jobUpdatedAt: friendlyDate(applicant.jobUpdatedAt),
    title: applicant.jobTitle,
    appliedAt: friendlyDate(applicant.appliedAt),
    devId: applicant.devId,
    fullName: applicant.devFullName,
    description: applicant.devDescription,
    primaryLangId: applicant.devPrimaryLangId,
    primaryLangName: applicant.devPrimaryLangName,
    secondaryLangId: applicant.devSecondaryLangId,
    secondaryLangName: applicant.devSecondaryLangName,
  };
}
