import { APPLY_JOB_URL, EntityId } from "./Api";

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
    const entity: EntityId = await result.json();
    return entity;
  }
  throw new Error("Failed to apply for job");
}
