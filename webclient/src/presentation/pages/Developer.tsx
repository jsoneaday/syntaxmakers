import "../../theme/developer.css";
import LeftMenu from "../components/LeftMenu";
import PromotedJobs from "../components/PromotedJobs";
import JobPost, { convert } from "../models/JobPost";
import Lister from "../components/Lister";
import { ChangeEvent, useEffect, useState } from "react";
import JobPreview from "../components/JobPreview";
import clipboard from "../../theme/assets/clipboard.png";
import clock from "../../theme/assets/wall-clock.png";
import { useProfile } from "../../domain/redux/profile/ProfileHooks";
import { getDeveloper } from "../../domain/repository/DeveloperRepo";
import { getJobsByDevProfile } from "../../domain/repository/JobRepo";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

export default function Developer() {
  const [jobData, setJobsData] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");
  const [profile, setProfile] = useProfile();

  useEffect(() => {
    getDeveloper(BigInt(1))
      .then((dev) => {
        setProfile(dev);
      })
      .catch((error) => {
        console.log("failed to get developer", error);
      });
  }, []);

  useEffect(() => {
    setJobsData([]);
    if (profile) {
      getJobsByDevProfile(profile.id)
        .then((jobs) => {
          setJobsData(
            jobs.map((job) => {
              return convert(job);
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
    <div className="dev-container">
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
  );
}
