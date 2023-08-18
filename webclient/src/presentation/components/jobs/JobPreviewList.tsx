import { ChangeEvent, MouseEvent, useEffect, useState } from "react";
import JobPost from "../../models/JobPost";
import Lister from "../Lister";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { getJobsByDevProfile } from "../../../domain/repository/JobRepo";
import { convert as convertJob } from "../../models/JobPost";
import clipboard from "../../theme/assets/clipboard.png";
import clock from "../../theme/assets/wall-clock.png";
import JobPreview from "./JobPreview";
import { DeveloperViewType } from "../../pages/Developer";

interface JobSearchProps {
  setLoginIsOpen: (isOpen: boolean) => void;
  onClickSwitchView: (developerViewType: DeveloperViewType) => void;
  currentDevViewType: DeveloperViewType;
}

export default function JobPreviewList({
  setLoginIsOpen,
  onClickSwitchView,
  currentDevViewType,
}: JobSearchProps) {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [profile, _setProfile] = useProfile();

  useEffect(() => {
    setJobsData([]);
    if (profile) {
      getJobsByDevProfile(profile.id)
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
    } else {
      setLoginIsOpen(true);
    }
  }, [profile]);

  const onSearchTxtChanged = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setSearchInput(e.target.value);
  };

  const onClickSelectJobItem = (_: MouseEvent<HTMLLIElement>) => {
    onClickSwitchView(
      currentDevViewType === DeveloperViewType.JobPreviewList
        ? DeveloperViewType.JobFullView
        : DeveloperViewType.JobPreviewList
    );
  };

  return (
    <div className="dev-main">
      <div className="dev-top">
        <div className="title-font dev-header">Developer job search</div>
        <div className="sub-title-font dev-sub-header">
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
      <div className="dev-post-preview-container">
        <ul>
          <Lister
            dataItems={jobData}
            elementCreator={(dataItem) => (
              <li
                key={dataItem.key}
                className="dev-preview-item"
                onClick={onClickSelectJobItem}
              >
                <JobPreview jobPost={dataItem} isSmall={false} />
              </li>
            )}
          />
        </ul>
      </div>
    </div>
  );
}
