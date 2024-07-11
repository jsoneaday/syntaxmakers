import { RoutePaths } from "../../../../App";
import { GroupLister } from "../../Lister";
import { Link } from "react-router-dom";
import { PreviewApplicant } from "./PreviewApplicant";
import { JobApplicantModel } from "../../../models/JobApplicantModel";

interface PreviewProfileListProps {
  applicants: JobApplicantModel[];
}

export default function PreviewApplicantList({
  applicants,
}: PreviewProfileListProps) {
  const onClickSelectJob = () => {};

  return (
    <div className="dev-post-preview-container">
      {applicants.length === 0 ? (
        <strong>No applicants found</strong>
      ) : (
        <GroupLister
          groupItems={applicants}
          elementCreator={(dataItem) => (
            <li
              key={dataItem.key}
              className="dev-preview-item"
              style={{ width: "100%" }}
            >
              <Link
                to={RoutePaths.ApplicantProfile}
                state={dataItem}
                onClick={onClickSelectJob}
              >
                <PreviewApplicant profile={dataItem} />
              </Link>
            </li>
          )}
        />
      )}
    </div>
  );
}
