import JobPost from "../../models/JobPost";
import "../../theme/post_preview.css";

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

  const timestamp = isSmall ? null : (
    <div className="small-font preview-timestamp">{jobPost.updatedAt}</div>
  );

  return (
    <div className="post-preview-container">
      <div className="preview-content">
        <div className={smallTitleFont}>{jobPost.title}</div>

        <div style={{ marginBottom: ".25em" }} className={smallSubTitleFont}>
          {jobPost.companyName}
        </div>
        <div style={{ padding: ".25em" }} className={smallNormalFont}>
          {jobPost.isRemote ? "Remote" : jobPost.countryName}
        </div>

        <div style={{ padding: ".25em" }}>
          <div className={"normal-font preview-item"}>
            Primary Language: <i>{`${jobPost.primaryLangName}`}</i>
          </div>
          <div className={"normal-font preview-item"}>
            Secondary Language: <i>{`${jobPost.secondaryLangName}`}</i>
          </div>
          <div className={"normal-font preview-item"}>
            Base Salary: <i>{`${jobPost.salary}`}</i>
          </div>
        </div>
        <div style={{ paddingLeft: ".25em" }}>{timestamp}</div>
      </div>
    </div>
  );
}
