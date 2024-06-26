import { ChangeEvent, MouseEvent, useEffect, useMemo, useState } from "react";
import JobPost from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import {
  Job,
  getJobsByDeveloper,
  getJobsBySearchTerms,
} from "../../../domain/repository/JobRepo";
import { convert as convertJob } from "../../models/JobPost";
import JobPreviewList from "../jobs/JobPreviewList";
import { useNavigationType } from "react-router-dom";
import { PrimaryButton } from "../controls/Buttons";
import { PAGE_SIZE } from "../../common/Paging";
import { Paging } from "../controls/Paging";

export default function DevJobPreviewList() {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [searchResultsMessage, setSearchResultsMessage] = useState("");
  const [profile, _setProfile] = useProfile();
  const navType = useNavigationType();
  const [pagingInit, setPagingInit] = useState<string | undefined>();

  useEffect(() => {
    queryUserJobs(0, true);
  }, [profile]);

  async function queryUserJobs(
    newOffset: number,
    setData: boolean
  ): Promise<Job[]> {
    let returnJobs: Job[] = [];
    if (!profile) return returnJobs;

    if (navType !== "POP") {
      setData && setJobsData([]);
      if (profile) {
        try {
          const jobs = await getJobsByDeveloper(
            profile.id,
            PAGE_SIZE,
            newOffset
          );
          const jobsData = jobs.map((job) => {
            return convertJob(job);
          });
          setData && setJobsData(jobsData);
          window.history.replaceState(jobsData, "");

          returnJobs = jobs;
        } catch (e) {
          console.log("failed to get jobs for current profile", e);
        }
      }
    } else {
      setJobsData(window.history.state);
    }

    if (!pagingInit) {
      setPagingInit(window.crypto.randomUUID());
    }
    setSearchResultsMessage("Your recommended jobs");
    return returnJobs;
  }

  const onSearchTxtChanged = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setSearchInput(e.target.value);
  };

  const onSearchJobs = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    await searchJobs(0, true);
  };

  async function searchJobs(
    newOffset: number,
    setData: boolean
  ): Promise<Job[]> {
    const searchTerms = searchInput.split(" ");
    const jobs = await getJobsBySearchTerms(searchTerms, PAGE_SIZE, newOffset);
    const jobsData = jobs.map((job) => {
      return convertJob(job);
    });
    setData && setJobsData(jobsData);

    window.history.replaceState(jobsData, "");
    setSearchResultsMessage(`Search results for terms: ${searchInput}`);
    return jobs;
  }

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
          <PrimaryButton onClick={onSearchJobs} disabled={searchBtnDisabled}>
            search
          </PrimaryButton>
        </div>
      </div>
      <div style={{ padding: "2em", width: "100%" }}>
        <div style={{ marginBottom: ".8em" }}>
          <strong>{searchResultsMessage}</strong>
        </div>
        <JobPreviewList jobPosts={jobData} />
        <Paging
          triggerInit={pagingInit}
          dataQuery={searchInput ? searchJobs : queryUserJobs}
        />
      </div>
    </div>
  );
}
