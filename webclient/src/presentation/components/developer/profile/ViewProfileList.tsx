import { RoutePaths } from "../../../../App";
import Lister from "../../Lister";
import { Link } from "react-router-dom";
import { PreviewProfile } from "./PreviewProfile";
import { JobApplicant } from "../../../models/JobApplicant";
import { useState } from "react";

export default function ViewProfileList() {
  const [applicants, _setApplicants] = useState<JobApplicant[]>([]);
  const onClickSelectJob = () => {};

  return (
    <div className="dev-post-preview-container">
      {applicants.length === 0 ? (
        <strong>No applicants found</strong>
      ) : (
        <Lister
          dataItems={applicants}
          elementCreator={(dataItem) => (
            <li key={dataItem.key} className="dev-preview-item">
              <Link
                to={RoutePaths.ApplicantProfile}
                state={dataItem}
                onClick={onClickSelectJob}
              >
                <PreviewProfile profile={dataItem} />
              </Link>
            </li>
          )}
        />
      )}
    </div>
  );
}
