import { JOBS_DEV_URL, JOBS_EMP_URL, JOB_UPDATE_URL } from "./Api";

export class Job {
  constructor(
    public id: number,
    public updatedAt: string, // comes from api as utc string
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

export interface JobFormState {
  id: number;
  employerId: number;
  title: string;
  description: string;
  isRemote: boolean;
  industryId: number;
  salaryId: number;
  primaryLangId: number;
  secondaryLangId?: number;
  countryId?: number;
}

export async function getJobsByDeveloper(
  id: string,
  pageSize: number = 20,
  lastOffset: number = 0
) {
  const result = await fetch(JOBS_DEV_URL, {
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

export async function getJobsByEmployer(
  id: string,
  pageSize: number = 20,
  lastOffset: number = 0
): Promise<Job[]> {
  const result = await fetch(JOBS_EMP_URL, {
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
    return await result.json();
  }
  return [];
}

export async function updateJobPost(
  jobFormState: JobFormState,
  access_token: string
) {
  console.log("jobFormState", jobFormState);

  const result = await fetch(JOB_UPDATE_URL, {
    method: "post",
    credentials: "include",
    cache: "no-store",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${access_token}`,
    },
    body: JSON.stringify({
      id: jobFormState.id,
      employerId: jobFormState.employerId,
      title: jobFormState.title,
      description: jobFormState.description,
      isRemote: jobFormState.isRemote,
      primaryLangId: jobFormState.primaryLangId,
      industryId: jobFormState.industryId,
      salaryId: jobFormState.salaryId,
      secondaryLangId: jobFormState.secondaryLangId,
      countryId: jobFormState.countryId,
    }),
  });

  if (result.ok) {
    if (result.status === 204) {
      return "";
    }
    return await result.json();
  }
  console.log(result.statusText);
  throw new Error("Failed to update job");
}
