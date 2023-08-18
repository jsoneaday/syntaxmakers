import JobPost from "../../models/JobPost";

interface JobFullViewProps {
  jobPost: JobPost;
}

export default function JobFullView({ jobPost }: JobFullViewProps) {
  return (
    <div className="dev-main">
      <div className="dev-top">top metadata</div>
      <div className="dev-post-preview-container">description body</div>
    </div>
  );
}
