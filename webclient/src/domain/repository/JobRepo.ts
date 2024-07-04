import {
  JOBS_APPLIED_URL,
  JOBS_DEV_URL,
  JOBS_EMP_URL,
  JOBS_SEARCH_URL,
  JOB_UPDATE_URL,
  JOB_URL,
  OutputId,
} from "./Api";

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

export async function getJob(id: number) {
  const result = await fetch(`${JOBS_APPLIED_URL}/${id}`, {
    method: "get",
  });

  if (result.ok) {
    const jobs: Job = await result.json();
    return jobs;
  }
  return null;
}

export async function getJobsByApplier(
  developerId: number,
  pageSize: number = 20,
  lastOffset: number = 0
) {
  const result = await fetch(JOBS_APPLIED_URL, {
    method: "post",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      id: developerId,
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

export async function getJobsBySearchTerms(
  searchTerms: string[],
  pageSize: number = 20,
  lastOffset: number = 0
) {
  const result = await fetch(JOBS_SEARCH_URL, {
    method: "post",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      searchTerms,
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
    const jobs: Job[] = await result.json();
    return jobs;
  }
  return [];
}

export async function insertJobPost(
  jobFormState: JobFormState,
  access_token: string
) {
  const response = await fetch(JOB_URL, {
    method: "post",
    credentials: "include",
    cache: "no-store",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${access_token}`,
    },
    body: JSON.stringify({
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

  if (response.ok) {
    if (response.status === 204) {
      return null;
    }
    const result: OutputId = await response.json();
    return result;
  }
  throw new Error("Failed to update job");
}

export async function updateJobPost(
  jobFormState: JobFormState,
  access_token: string
) {
  console.log("jobFormState.description", jobFormState.description);
  const response = await fetch(JOB_UPDATE_URL, {
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
      countryId: jobFormState.countryId,
      primaryLangId: jobFormState.primaryLangId,
      secondaryLangId: jobFormState.secondaryLangId,
      industryId: jobFormState.industryId,
      salaryId: jobFormState.salaryId,
    }),
  });

  if (response.ok) {
    if (response.status === 204) {
      return null;
    }
    return await response.json();
  }
  throw new Error("Failed to update job");
}
