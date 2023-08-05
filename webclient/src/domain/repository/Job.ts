import { JOBS_URL } from "./Api";

export class Job {
  constructor(
    public id: bigint,
    public updated_at: Date,
    public employer_id: bigint,
    public title: string,
    public description: string,
    public isRemote: boolean,
    public countryId: bigint | undefined,
    public primaryLangId: bigint,
    public secondaryLangId: bigint,
    public industryId: bigint,
    public salaryId: bigint
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
    return jobs;
  }
  return [];
}
