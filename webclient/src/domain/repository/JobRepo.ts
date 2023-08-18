import { JOBS_URL } from "./Api";

export class Job {
  constructor(
    public id: string,
    public updatedAt: string, // comes from api as utc string
    public employerId: string,
    public employerName: string,
    public companyId: string,
    public companyName: string,
    public title: string,
    public description: string,
    public isRemote: boolean,
    public primaryLangId: string,
    public primaryLangName: string,
    public secondaryLangId: string,
    public secondaryLangName: string,
    public industryId: string,
    public industryName: string,
    public salaryId: string,
    public salary: string,
    public companyLogo?: ArrayBuffer, // normal format for file data received over wire
    public countryId?: string,
    public countryName?: string
  ) {}
}

export async function getJobsByDevProfile(
  id: string,
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
    return jobs;
  }
  return [];
}
