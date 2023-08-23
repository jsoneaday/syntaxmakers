import { useLocation } from "react-router-dom";
import JobPost from "../../models/JobPost";
import { useEffect, useState } from "react";
import "../../theme/job_full_view.css";
import { appendPlusLargeCurrency } from "../../common/CurrencyFormatter";
import flag from "../../theme/assets/flag.png";
import similar from "../../theme/assets/similar.png";
import GoBack from "../../components/navigation/GoBack";
import TextEditor from "../../components/textEditor/TextEditor";

export default function JobFullview() {
  const { state } = useLocation();
  const [jobPost, setJobPost] = useState<JobPost>();
  const [salary, setSalary] = useState("");

  useEffect(() => {
    const currentJobPost = state as JobPost;
    setJobPost(currentJobPost);
    setSalary(appendPlusLargeCurrency(currentJobPost?.salary || ""));
  }, [state]);

  return (
    <div className="userhome-main" style={{ width: "738px", margin: "auto" }}>
      <div
        style={{ paddingTop: "2em", paddingLeft: "2em", paddingRight: "2em" }}
      >
        <GoBack label="dev home" />
      </div>
      <div
        className="opposites"
        style={{ paddingTop: "2em", paddingLeft: "2em", paddingRight: "2em" }}
      >
        <div className="userhome-top">
          <div className="opposites">
            <div className="title-font">{jobPost?.title}</div>
          </div>

          <div className="opposites">
            <div className="job-full-view-subtitle">
              <div className="sub-title-font job-full-view-subtitle-item-primary">
                {jobPost?.companyName}
              </div>
              <div className="sub-title-font job-full-view-subtitle-item-primary">
                {jobPost?.isRemote ? "Remote" : jobPost?.countryName}
              </div>
              <div className="sub-title-font job-full-view-subtitle-item-primary">
                {jobPost?.updatedAt}
              </div>
            </div>
          </div>

          <div className="job-full-view-subtitle-item-secondary">
            {`Contact: ${jobPost?.employerName}`}
          </div>
          <div className="job-full-view-subtitle-item-secondary">
            {`Primary Language: ${jobPost?.primaryLangName}`}
          </div>
          {jobPost?.secondaryLangName &&
          jobPost?.secondaryLangName != jobPost?.primaryLangName ? (
            <div className="job-full-view-subtitle-item-secondary">
              {`Secondary Language: ${jobPost?.secondaryLangName}`}
            </div>
          ) : null}
          <div className="job-full-view-subtitle-item-secondary">
            {`Industry: ${jobPost?.industryName}`}
          </div>
          <div className="job-full-view-subtitle-item-secondary">
            {`Base Salary: ${salary}`}
          </div>
        </div>
        <div
          className="stack"
          style={{ alignItems: "flex-end", textAlign: "right" }}
        >
          <button
            className="primary-btn small-btn"
            style={{ marginBottom: ".5em" }}
          >
            apply
          </button>
          <button className="secondary-btn small-btn">save</button>
          <img
            src={flag}
            className="job-icon"
            style={{ marginTop: "1em" }}
            title="inappropriate"
          />
          <img
            src={similar}
            className="job-icon"
            style={{ marginTop: ".50em" }}
            title="similar jobs"
          />
        </div>
      </div>

      <div
        className="normal-font dev-post-preview-container"
        style={{
          paddingTop: "2em",
          paddingLeft: "2em",
          paddingRight: "2em",
          paddingBottom: "2em",
        }}
      >
        <span className="title-font" style={{ marginBottom: "1em" }}>
          Description
        </span>
        <TextEditor
          initialValue={[
            {
              type: "paragraph",
              children: [{ text: "A line of text in a paragraph." }],
            },
          ]}
          readOnly={true}
        />
      </div>
    </div>
  );
}
