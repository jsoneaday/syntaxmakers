import {
  APPLY_JOB_URL,
  DEV_APPLIED_JOB_URL,
  OutputBool,
  OutputId,
} from "./Api";

export class JobApplied {
  constructor(
    public id: number,
    public updatedAt: string, // comes from api as utc string
    public jobAppliedAt: string,
    public employerId: number,
    public employerName: string,
    public companyId: number,
    public companyName: string,
    public title: string,
    public description: string,
    public isRemote: boolean,
    public primaryLangId: number,
    public primaryLangName: string,
    public secondaryLangId: number,
    public secondaryLangName: string,
    public industryId: number,
    public industryName: string,
    public salaryId: number,
    public salary: string,
    public companyLogo?: ArrayBuffer, // normal format for file data received over wire
    public countryId?: number,
    public countryName?: string
  ) {}
}

export async function applyJob(
  jobId: number,
  developerId: number,
  access_token: string
) {
  const result = await fetch(APPLY_JOB_URL, {
    method: "POST",
    credentials: "include",
    cache: "no-store",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${access_token}`,
    },
    body: JSON.stringify({
      jobId,
      developerId,
    }),
  });

  if (result.ok) {
    const entity: OutputId = await result.json();
    return entity;
  }
  throw new Error("Failed to apply for job");
}

export async function developerAppliedToJob(
  jobId: number,
  developerId: number
) {
  const response = await fetch(DEV_APPLIED_JOB_URL, {
    method: "post",
    headers: {
      "Content-Type": "application/json",
    },
    cache: "no-store",
    body: JSON.stringify({
      jobId,
      developerId,
    }),
  });

  if (response.ok) {
    const output: OutputBool = await response.json();

    return output.result;
  }
  return false;
}
