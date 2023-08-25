import { ChangeEvent, useEffect, useState } from "react";
import JobPost from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { getJobsByDeveloper } from "../../../domain/repository/JobRepo";
import { convert as convertJob } from "../../models/JobPost";
import clipboard from "../../theme/assets/clipboard.png";
import clock from "../../theme/assets/wall-clock.png";
import JobPreviewList from "../jobs/JobPreviewList";

export default function DevJobPreviewList() {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [profile, _setProfile] = useProfile();

  useEffect(() => {
    setJobsData([]);
    if (profile) {
      getJobsByDeveloper(profile.id)
        .then((jobs) => {
          setJobsData(
            jobs.map((job) => {
              return convertJob(job);
            })
          );
        })
        .catch((error) => {
          console.log("failed to get jobs for current profile", error);
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
        <div className="title-font userhome-header">Developer job search</div>
        <div className="sub-title-font userhome-sub-header">
          Enter your preferences to find your next job
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
      <div className="info-band">
        <img className="dev-info-band-icon" src={clipboard} />
        Result count 231
        <img
          className="dev-info-band-icon"
          style={{ marginLeft: "1.5em" }}
          src={clock}
        />
        Date jun 16, 2023
      </div>
      <JobPreviewList jobPosts={jobData} />
    </div>
  );
}
