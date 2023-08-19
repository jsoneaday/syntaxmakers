import { useLocation } from "react-router-dom";
import JobPost from "../models/JobPost";
import { useEffect, useState } from "react";
import "../theme/job_full_view.css";
import { appendPlusLargeCurrency } from "../common/CurrencyFormatter";
import Layout from "../components/Layout";

export default function JobFullView() {
  const { state } = useLocation();
  const [jobPost, setJobPost] = useState<JobPost>();
  const [salary, setSalary] = useState("");

  useEffect(() => {
    setJobPost(state);
    setSalary(appendPlusLargeCurrency(state?.salary || ""));
  }, [state]);

  return (
    <Layout>
      <div className="dev-main" style={{ width: "738px", margin: "auto" }}>
        <div className="dev-top">
          <div className="title-font">{jobPost?.title}</div>
          <div className="job-full-view-subtitle">
            <div className="sub-title-font job-full-view-subtitle-item-primary">
              {jobPost?.companyName}
            </div>
            <div className="job-full-view-subtitle-item-primary">
              {jobPost?.isRemote ? "Remote" : jobPost?.countryName}
            </div>
            <div className="job-full-view-subtitle-item-primary">
              {jobPost?.updatedAt}
            </div>
          </div>
          <div className="job-full-view-subtitle-item-secondary">
            {`Contact: ${jobPost?.employerName}`}
          </div>
          <div className="job-full-view-subtitle-item-secondary">
            {`Primary Language: ${jobPost?.primaryLangName}`}
          </div>
          {jobPost?.secondaryLangName ? (
            <div className="job-full-view-subtitle-item-secondary">
              {`Secondary Language: ${jobPost?.secondaryLangName}`}
            </div>
          ) : null}
          <div className="job-full-view-subtitle-item-secondary">
            {`Industry: ${jobPost?.industryName}`}
          </div>
          <div className="job-full-view-subtitle-item-secondary">
            {`Contact: ${salary}`}
          </div>
        </div>
        <div className="dev-post-preview-container">{jobPost?.description}</div>
      </div>
    </Layout>
  );
}
