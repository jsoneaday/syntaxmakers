import { useState } from "react";
import { Paging } from "../controls/Paging";
import JobPreviewList from "../jobs/JobPreviewList";
import JobPost from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { getJobsByApplier } from "../../../domain/repository/JobRepo";
import { convert as convertJob } from "../../../presentation/models/JobPost";
import { PAGE_SIZE } from "../../common/Paging";

export function DevAppliedJobsList() {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [profile] = useProfile();

  const getAppliedJobs = async (newOffset: number, setData: boolean) => {
    if (!profile) return [];

    const jobs = await getJobsByApplier(
      Number(profile.id),
      PAGE_SIZE,
      newOffset
    );
    const jobsData = jobs.map((job) => {
      return convertJob(job);
    });
    setData && setJobsData(jobsData);

    return jobsData;
  };

  return (
    <div className="userhome-main">
      <header className="header-container job-full-view-header">
        <strong>Applied Jobs</strong>
      </header>
      <div style={{ padding: "2em", width: "100%" }}>
        <JobPreviewList jobPosts={jobData} />
        <Paging dataQuery={getAppliedJobs} />
      </div>
    </div>
  );
}
