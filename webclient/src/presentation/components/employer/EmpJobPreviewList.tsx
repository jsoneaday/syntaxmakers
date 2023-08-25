import { ChangeEvent, useEffect, useState } from "react";
import JobPost, { convert } from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import JobPreviewList from "../jobs/JobPreviewList";
import "../../theme/emphome.css";
import { getJobsByEmployer } from "../../../domain/repository/JobRepo";

export default function EmpJobPreviewList() {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [profile, _setProfile] = useProfile();

  useEffect(() => {
    setJobsData([]);
    if (profile) {
      getJobsByEmployer(profile.id, 40, 0)
        .then((jobs) => {
          setJobsData(jobs.map((job) => convert(job)));
        })
        .catch((err) => {
          console.log("Failed to get employer jobs", err);
        });
    }
  }, [profile]);

  const onSearchTxtChanged = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setSearchInput(e.target.value);
  };

  return (
    <div className="userhome-main">
      <div className="userhome-top" style={{ padding: "2em" }}>
        <div className="title-font userhome-header">
          Search your existing jobs posts
        </div>
        <div className="search-header">
          <input
            className="search-input"
            type="text"
            value={searchInput}
            onChange={onSearchTxtChanged}
          />
          <button className="primary-btn">search</button>
        </div>
      </div>
      <div className="emphome-job-list">
        <JobPreviewList jobPosts={jobData} />
      </div>
    </div>
  );
}
