import JobPost from "../../models/JobPost";
import Lister from "../Lister";
import JobPreview from "./JobPreview";
import { Link } from "react-router-dom";

interface JobPreviewListProps {
  jobPosts: JobPost[];
}

export default function JobPreviewList({ jobPosts }: JobPreviewListProps) {
  return (
    <div className="dev-post-preview-container" style={{ padding: "2em" }}>
      <ul>
        <Lister
          dataItems={jobPosts}
          elementCreator={(dataItem) => (
            <li key={dataItem.key} className="dev-preview-item">
              <Link to="/jobfullview" state={dataItem}>
                <JobPreview jobPost={dataItem} isSmall={false} />
              </Link>
            </li>
          )}
        />
      </ul>
    </div>
  );
}
