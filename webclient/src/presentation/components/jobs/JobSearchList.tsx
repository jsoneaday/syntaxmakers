import { ChangeEvent, useMemo, useState } from "react";
import JobPost from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import {
  getEmployerJobsBySearchTerms,
  getJobsByDeveloper,
  getJobsByEmployer,
  getJobsBySearchTerms,
} from "../../../domain/repository/JobRepo";
import { convert as convertJob } from "../../models/JobPost";
import JobPreviewList from "./JobPreviewList";
import { Link, useParams } from "react-router-dom";
import { PrimaryButton } from "../controls/Buttons";
import { PAGE_SIZE } from "../../common/Paging";
import { Paging } from "../controls/Paging";
import { RoutePaths } from "../../../App";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";

interface JobSearchListProps {
  userType: UiDevOrEmployer;
}

export default function JobSearchList({ userType }: JobSearchListProps) {
  const { search } = useParams();
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState(search || "");
  const [searchResultsMessage, setSearchResultsMessage] = useState("");
  const [profile, _setProfile] = useProfile();

  async function queryUserJobs(
    newOffset: number,
    setData: boolean
  ): Promise<JobPost[]> {
    let returnJobs: JobPost[] = [];
    if (!profile) return returnJobs;

    setData && setJobsData([]);
    if (profile) {
      if (userType === UiDevOrEmployer.Developer) {
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

          returnJobs = jobsData;
        } catch (e) {
          console.log("failed to get jobs for current profile", e);
        }

        setSearchResultsMessage("Your recommended jobs");
      } else {
        try {
          const jobs = await getJobsByEmployer(
            profile.id,
            PAGE_SIZE,
            newOffset
          );
          const jobsData = jobs.map((job) => {
            return convertJob(job);
          });

          setData && setJobsData(jobsData);

          returnJobs = jobsData;
        } catch (e) {
          console.log("failed to get jobs for current profile", e);
        }

        setSearchResultsMessage("Your posted jobs");
      }
    }

    return returnJobs;
  }

  const onSearchTxtChanged = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setSearchInput(e.target.value);
  };

  async function searchJobs(
    newOffset: number,
    setData: boolean
  ): Promise<JobPost[]> {
    const searchTerms = search?.split(" ");
    if (!searchTerms) throw new Error("Search terms are missing");

    let jobsData: JobPost[] = [];
    if (userType === UiDevOrEmployer.Developer) {
      const jobs = await getJobsBySearchTerms(
        searchTerms,
        PAGE_SIZE,
        newOffset
      );
      jobsData = jobs.map((job) => {
        return convertJob(job);
      });
      setData && setJobsData(jobsData);
    } else {
      if (profile) {
        const jobs = await getEmployerJobsBySearchTerms(
          profile.id,
          searchTerms,
          PAGE_SIZE,
          newOffset
        );
        jobsData = jobs.map((job) => {
          return convertJob(job);
        });
        setData && setJobsData(jobsData);
      } else {
        setSearchResultsMessage("You must be logged in to search");
        return [];
      }
    }

    console.log("searchJobs", newOffset, setData, jobsData);
    setSearchResultsMessage(`Search results for terms: ${search}`);
    return jobsData;
  }

  const searchBtnDisabled = useMemo(() => {
    return searchInput.length > 1 ? false : true;
  }, [searchInput]);

  return (
    <div className="userhome-main">
      <header className="header-container job-full-view-header">
        <strong>
          {userType === UiDevOrEmployer.Developer
            ? "Developer Job Search"
            : "Posted Jobs Search"}
        </strong>
      </header>
      <div className="userhome-top" style={{ padding: "2em" }}>
        <div className="sub-title-font userhome-sub-header">
          {userType === UiDevOrEmployer.Developer
            ? "Enter your preferences to find your next job"
            : "Search your job postings"}
        </div>
        <div className="search-header">
          <input
            className="search-input"
            type="text"
            value={searchInput}
            onChange={onSearchTxtChanged}
          />
          <PrimaryButton disabled={searchBtnDisabled}>
            <Link
              to={
                userType === UiDevOrEmployer.Developer
                  ? `${RoutePaths.DevJobSearch}/${searchInput}`
                  : `${RoutePaths.EmpJobPosts}/${searchInput}`
              }
            >
              search
            </Link>
          </PrimaryButton>
        </div>
      </div>
      <div style={{ padding: "2em", width: "100%" }}>
        <div style={{ marginBottom: ".8em" }}>
          <strong>{searchResultsMessage}</strong>
        </div>
        <JobPreviewList jobPosts={jobData} />
        <Paging dataQuery={search ? searchJobs : queryUserJobs} />
      </div>
    </div>
  );
}
