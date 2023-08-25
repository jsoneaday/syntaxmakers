import { useLocation } from "react-router-dom";
import JobPost from "../../models/JobPost";
import { ChangeEvent, useEffect, useState } from "react";
import "../../theme/job_full_view.css";
import { appendPlusLargeCurrency } from "../../common/CurrencyFormatter";
import flag from "../../theme/assets/flag.png";
import similar from "../../theme/assets/similar.png";
import GoBack from "../../components/navigation/GoBack";
import TextEditor from "../../components/textEditor/TextEditor";
import DropDown from "../controls/DropDown";

interface JobFullviewProps {
  readOnly: boolean;
}

export default function JobFullview({ readOnly }: JobFullviewProps) {
  const { state } = useLocation();
  const [jobPost, setJobPost] = useState<JobPost>();
  const [salary, setSalary] = useState("");
  const [title, setTitle] = useState("");
  const [companyName, setCompanyName] = useState("");
  const [isRemote, setIsRemote] = useState(false);
  const [updatedAt, setUpdatedAt] = useState("");

  useEffect(() => {
    const currentJobPost = state as JobPost;
    setJobPost(currentJobPost);
    setSalary(appendPlusLargeCurrency(currentJobPost?.salary || ""));
  }, [state]);

  const onChangeTitle = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
  };

  return (
    <form className="userhome-main" style={{ margin: "auto" }}>
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
            {readOnly ? (
              <div className="title-font">{jobPost?.title}</div>
            ) : (
              <div className="left-align">
                <label htmlFor="job-title-input" style={{ marginRight: "1em" }}>
                  Title
                </label>
                <input
                  id="job-title-input"
                  type="text"
                  value={title}
                  onChange={onChangeTitle}
                  className="input"
                />
              </div>
            )}
          </div>

          <div className="opposites">
            <div className="job-full-view-subtitle">
              {readOnly ? (
                <div className="sub-title-font job-full-view-subtitle-item-primary">
                  {jobPost?.companyName}
                </div>
              ) : (
                <DropDown optionItems={[{ name: "Company A", value: "1" }]} />
              )}
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
          padding: "2em",
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
          readOnly={readOnly}
        />
      </div>
    </form>
  );
}
