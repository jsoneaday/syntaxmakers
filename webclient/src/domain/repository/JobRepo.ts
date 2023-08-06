import { JOBS_URL } from "./Api";

export class Job {
  constructor(
    public id: bigint,
    public updatedAt: string, // comes from api as utc string
    public employerId: bigint,
    public employerName: string,
    public companyId: bigint,
    public companyName: string,
    public title: string,
    public description: string,
    public isRemote: boolean,
    public primaryLangId: bigint,
    public primaryLangName: string,
    public secondaryLangId: bigint,
    public secondaryLangName: string,
    public industryId: bigint,
    public industryName: string,
    public salaryId: bigint,
    public salary: string,
    public companyLogo?: ArrayBuffer, // normal format for file data received over wire
    public countryId?: bigint,
    public countryName?: string
  ) {}
}

export async function getJobsByDevProfile(
  id: bigint,
  pageSize: number = 20,
  lastOffset: number = 0
) {
  const result = await fetch(JOBS_URL, {
    method: "post",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      id,
      pageSize,
      lastOffset,
    }),
  });

  if (result.ok) {
    const jobs: Job[] = await result.json();
    console.log("jobs", jobs);
    return jobs;
  }
  return [];
}
