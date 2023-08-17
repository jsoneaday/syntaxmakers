import "../../presentation/theme/developer.css";
import LeftMenu from "../components/LeftMenu";
import PromotedJobs from "../components/PromotedJobs";
import JobPost, { convert as convertJob } from "../models/JobPost";
import Lister from "../components/Lister";
import { ChangeEvent, useEffect, useState } from "react";
import JobPreview from "../components/JobPreview";
import clipboard from "../../presentation/theme/assets/clipboard.png";
import clock from "../../presentation/theme/assets/wall-clock.png";
import { useProfile } from "../common/redux/profile/ProfileHooks";
import { getJobsByDevProfile } from "../../domain/repository/JobRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import Login from "../components/authentication/Login";
import { DevOrEmployer } from "../models/DevOrEmployer";

export default function Developer() {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [profile, _setProfile] = useProfile();
  const [loginIsOpen, setLoginIsOpen] = useState(false);

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

  const toggleOpen = () => {
    setLoginIsOpen(!loginIsOpen);
  };

  return (
    <>
      <Login
        devOrEmployer={DevOrEmployer.Developer}
        isOpen={loginIsOpen}
        toggleOpen={toggleOpen}
      />
      <div className="dev-container" data-testid="developer-page">
        <LeftMenu />
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
                  <li key={dataItem.key} className="dev-preview-item">
                    <JobPreview jobPost={dataItem} isSmall={false} />
                  </li>
                )}
              />
            </ul>
          </div>
        </div>
        <PromotedJobs posts={[]} />
      </div>
    </>
  );
}
