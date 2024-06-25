import { ChangeEvent, MouseEvent, useEffect, useMemo, useState } from "react";
import JobPost from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import {
  getJobsByDeveloper,
  getJobsBySearchTerms,
} from "../../../domain/repository/JobRepo";
import { convert as convertJob } from "../../models/JobPost";
import JobPreviewList from "../jobs/JobPreviewList";
import { useNavigationType } from "react-router-dom";
import { PrimaryButton } from "../controls/Buttons";
import { PAGE_SIZE } from "../../common/Paging";

export default function DevJobPreviewList() {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [profile, _setProfile] = useProfile();
  const navType = useNavigationType();
  // an offset is equivalent to skip
  const [offset, setOffset] = useState(0);

  useEffect(() => {
    if (navType !== "POP") {
      setJobsData([]);
      if (profile) {
        getJobsByDeveloper(profile.id, PAGE_SIZE, offset)
          .then((jobs) => {
            const jobsData = jobs.map((job) => {
              return convertJob(job);
            });
            setJobsData(jobsData);
            window.history.replaceState(jobsData, "");
          })
          .catch((error) => {
            console.log("failed to get jobs for current profile", error);
          });
      }
    } else {
      setJobsData(window.history.state);
    }
  }, [profile]);

  const onSearchTxtChanged = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setSearchInput(e.target.value);
  };

  const searchJobs = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    const searchTerms = searchInput.split(" ");
    const jobs = await getJobsBySearchTerms(searchTerms, PAGE_SIZE, offset);
    const jobsData = jobs.map((job) => {
      return convertJob(job);
    });
    setJobsData(jobsData);
    window.history.replaceState(jobsData, "");
  };

  const searchBtnDisabled = useMemo(() => {
    return searchInput.length > 2 ? false : true;
  }, [searchInput]);

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
          <PrimaryButton onClick={searchJobs} disabled={searchBtnDisabled}>
            search
          </PrimaryButton>
        </div>
      </div>
      <JobPreviewList jobPosts={jobData} />
    </div>
  );
}
