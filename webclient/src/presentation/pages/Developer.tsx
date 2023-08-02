import "../../theme/developer.css";
import LeftMenu from "../components/LeftMenu";
import PromotedJobs from "../components/PromotedJobs";
import JobPost from "../models/JobPost";
import Lister from "../components/Lister";
import { useState } from "react";
import JobPreview from "../components/JobPreview";
import clipboard from "../../theme/assets/clipboard.png";
import clock from "../../theme/assets/wall-clock.png";

export default function Developer() {
  const [dataItems, setDataItems] = useState<JobPost[]>([]);
  const [searchInput, setSearchInput] = useState("");

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
            <input className="search-input" type="text" value={searchInput} />
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
              dataItems={dataItems}
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
