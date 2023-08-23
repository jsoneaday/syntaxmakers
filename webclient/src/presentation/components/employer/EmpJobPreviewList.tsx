import { ChangeEvent, useEffect, useState } from "react";
import JobPost from "../../models/JobPost";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import JobPreviewList from "../jobs/JobPreviewList";
import "../../theme/emphome.css";

interface JobPreviewListProps {
  setLoginIsOpen: (isOpen: boolean) => void;
}

export default function EmpJobPreviewList({
  setLoginIsOpen,
}: JobPreviewListProps) {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [profile, _setProfile] = useProfile();

  useEffect(() => {
    setJobsData([]);
    if (profile) {
      // todo: get employers posted jobs
    } else {
      setLoginIsOpen(true);
    }
  }, [profile]);

  const onSearchTxtChanged = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setSearchInput(e.target.value);
  };

  return (
    <div className="userhome-main">
      <div className="userhome-top" style={{ padding: "2em" }}>
        <div className="emphome-post-container" style={{ marginBottom: "2em" }}>
          <div
            className="title-font userhome-header"
            style={{ marginRight: "1em" }}
          >
            Post a new job
          </div>
          <button className="primary-btn" style={{ width: "10em" }}>
            post
          </button>
        </div>

        <div className="title-font userhome-header">Search your jobs posts</div>
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
      <JobPreviewList jobPosts={jobData} />
    </div>
  );
}
