import { RoutePaths } from "../../../App";
import { startViewTransition } from "../../common/transitions/ViewTransition";
import JobPost from "../../models/JobPost";
import Lister from "../Lister";
import JobPreview from "./JobPreview";
import { Link } from "react-router-dom";

interface JobPreviewListProps {
  jobPosts: JobPost[];
}

export default function JobPreviewList({ jobPosts }: JobPreviewListProps) {
  const onClickSelectJob = () => {
    startViewTransition(() => {});
  };

  return (
    <div className="dev-post-preview-container" style={{ padding: "2em" }}>
      <ul>
        <Lister
          dataItems={jobPosts}
          elementCreator={(dataItem) => (
            <li key={dataItem.key} className="dev-preview-item">
              <Link
                to={RoutePaths.DevJobFullView}
                state={dataItem}
                onClick={onClickSelectJob}
              >
                <JobPreview jobPost={dataItem} isSmall={false} />
              </Link>
            </li>
          )}
        />
      </ul>
    </div>
  );
}
