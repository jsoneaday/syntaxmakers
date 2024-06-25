import { RoutePaths } from "../../../App";
import { useDevOrEmployer } from "../../common/redux/devOrEmployer/DevOrEmployerHooks";
import { startViewTransition } from "../../common/transitions/ViewTransition";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import JobPost from "../../models/JobPost";
import Lister from "../Lister";
import JobPreview from "./JobPreview";
import { Link } from "react-router-dom";

interface JobPreviewListProps {
  jobPosts: JobPost[];
}

export default function JobPreviewList({ jobPosts }: JobPreviewListProps) {
  const [devOrEmp] = useDevOrEmployer();

  const onClickSelectJob = () => {
    startViewTransition(() => {});
  };

  return (
    <div className="dev-post-preview-container" style={{ padding: "2em" }}>
      {jobPosts.length === 0 ? (
        <strong>No jobs found</strong>
      ) : (
        <Lister
          dataItems={jobPosts}
          elementCreator={(dataItem) => (
            <li key={dataItem.key} className="dev-preview-item">
              <Link
                to={
                  devOrEmp === DevOrEmployer.Developer
                    ? RoutePaths.DevJobFullView
                    : RoutePaths.EmpJobFullView
                }
                state={dataItem}
                onClick={onClickSelectJob}
              >
                <JobPreview jobPost={dataItem} isSmall={false} />
              </Link>
            </li>
          )}
        />
      )}
    </div>
  );
}
