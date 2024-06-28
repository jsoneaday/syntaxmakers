import { useEffect, useState } from "react";
import { Paging } from "../controls/Paging";
import JobPreviewList from "../jobs/JobPreviewList";
import JobPost from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { getJobsByApplier } from "../../../domain/repository/JobRepo";
import { convert as convertJob } from "../../../presentation/models/JobPost";

export function DevAppliedJobsList() {
  const [pagingInit, setPagingInit] = useState<string | undefined>();
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [profile, _setProfile] = useProfile();

  useEffect(() => {
    getAppliedJobs();
  }, []);

  const getAppliedJobs = async () => {
    console.log("profile", profile);
    if (!profile) return [];

    const jobs = await getJobsByApplier(Number(profile.id));
    const jobsData = jobs.map((job) => {
      return convertJob(job);
    });
    setJobsData(jobsData);

    if (!pagingInit) {
      setPagingInit(window.crypto.randomUUID());
    }
    return jobsData;
  };

  return (
    <div className="userhome-main">
      <header className="header-container job-full-view-header">
        <strong>Applied Jobs</strong>
      </header>
      <div style={{ padding: "2em", width: "100%" }}>
        <JobPreviewList jobPosts={jobData} />
        <Paging triggerInit={pagingInit} dataQuery={getAppliedJobs} />
      </div>
    </div>
  );
}
