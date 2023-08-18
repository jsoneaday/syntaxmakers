import JobPost from "../models/JobPost";
import JobPreview from "./jobs/JobPreview";
import Lister from "./Lister";
import "../theme/promoted_jobs.css";

interface PromotedJobsProps {
  posts: JobPost[];
}

export default function PromotedJobs({ posts }: PromotedJobsProps) {
  return (
    <div className="panel-col job-menu-container">
      <div className="title-font">Promoted jobs</div>
      <ul>
        <Lister
          dataItems={posts}
          elementCreator={(item) => (
            <li key={item.key} className="dev-preview-item">
              <JobPreview jobPost={item} isSmall={true} />
            </li>
          )}
        />
      </ul>
    </div>
  );
}
