import JobPost from "../models/JobPost";
import "../theme//post_preview.css";

interface JobPreviewProps {
  jobPost: JobPost;
  isSmall: boolean;
}

export default function JobPreview({ jobPost, isSmall }: JobPreviewProps) {
  const smallTitleFont = isSmall
    ? "title-font preview-small-title-font"
    : "title-font";

  const smallSubTitleFont = isSmall
    ? "sub-title-font preview-small-sub-title-font"
    : "sub-title-font";

  const smallNormalFont = isSmall
    ? "normal-font preview-small-normal-font"
    : "normal-font";

  const icon = isSmall ? null : (
    <img
      className="preview-icon"
      src={jobPost.companyLogo ? URL.createObjectURL(jobPost.companyLogo) : ""}
    />
  );

  const timestamp = isSmall ? null : (
    <div className="small-font preview-timestamp">{jobPost.updatedAt}</div>
  );

  return (
    <div className="post-preview-container">
      {icon}
      <div className="preview-content">
        <div className={smallTitleFont}>{jobPost.title}</div>
        <div className={smallSubTitleFont}>{jobPost.companyName}</div>
        <div className={smallNormalFont}>
          {jobPost.isRemote ? "Remote" : jobPost.countryName}
        </div>
        <div className={"normal-font preview-salary"}>
          Base Salary: <i>{`$${jobPost.salary}`}</i>
        </div>
        {timestamp}
      </div>
    </div>
  );
}
