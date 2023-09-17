import { useLocation } from "react-router-dom";
import JobPost from "../../models/JobPost";
import { ChangeEvent, useEffect, useState } from "react";
import "../../theme/job_full_view.css";
import flag from "../../theme/assets/flag.png";
import similar from "../../theme/assets/similar.png";
import GoBack from "../navigation/GoBack";
import TextEditor from "../textEditor/TextEditor";
import DropDown from "../controls/DropDown";
import Checkbox from "../controls/Checkbox";
import { useDevOrEmployer } from "../../common/redux/devOrEmployer/DevOrEmployerHooks";
import { DevOrEmployer } from "../../models/DevOrEmployer";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";
import {
  JobPostOptions,
  getJobPostOptions,
} from "../../models/JobFullviewModel";

interface FormState {
  id: string;
  updatedAt: string;
  employerId: string;
  employerName: string;
  title: string;
  description: string;
  isRemote: boolean;
  countryId: string;
  companyId: string;
  industryId: string;
  salaryId: string;
  primaryLangId: string;
  secondaryLangId?: string;
}

type JobPostDisplayObject = {
  title: JSX.Element;
  companyName: JSX.Element;
  isRemoteOrCountry: JSX.Element;
  updatedAt: JSX.Element | null;
  buttons: JSX.Element;
  employerName: JSX.Element;
  primaryLang: JSX.Element;
  secondaryLang: JSX.Element | null;
  industry: JSX.Element;
  salary: JSX.Element;
};

interface JobFullviewProps {
  readOnly: boolean;
}

export default function JobFullview({ readOnly }: JobFullviewProps) {
  const { state: routeJobPost } = useLocation();
  const [jobPost, setJobPost] = useState<JobPost>({
    key: uuidv4(),
    id: "",
    updatedAt: "",
    title: "",
    description: "",
    employerId: "",
    employerName: "",
    companyId: "",
    companyName: "",
    isRemote: false,
    primaryLangId: "",
    primaryLangName: "",
    secondaryLangId: "",
    secondaryLangName: "",
    industryId: "",
    industryName: "",
    salaryId: "",
    salary: "",
  });
  const [jobPostDisplayComponents, setJobPostDisplayComponents] =
    useState<JobPostDisplayObject>();
  const [formValues, setFormValues] = useState<FormState>({
    id: "",
    updatedAt: "",
    employerId: "",
    employerName: "",
    title: "",
    description: "",
    isRemote: false,
    countryId: "",
    companyId: "",
    industryId: "",
    salaryId: "",
    primaryLangId: "",
    secondaryLangId: "",
  });
  const [devOrEmp] = useDevOrEmployer();

  useEffect(() => {
    let currentJobPost: JobPost | undefined = undefined;
    if (routeJobPost) {
      currentJobPost = routeJobPost as JobPost;
      setJobPost(currentJobPost);
      console.log("jobPost", currentJobPost);
    }
  }, [routeJobPost]);

  useEffect(() => {
    if (!readOnly) {
      getJobPostOptions().then((jobPostOptions) => {
        const jobPostDisplayComponentItems = getJobPostDisplayComponents(
          jobPostOptions,
          jobPost
        );
        setJobPostDisplayComponents(jobPostDisplayComponentItems);
      });
    }
  }, [jobPost]);

  const getJobPostDisplayComponents = (
    jobPostOptions: JobPostOptions,
    jobPostObject: JobPost | undefined
  ) => {
    let title: JSX.Element;
    let companyName: JSX.Element;
    let isRemoteOrCountry: JSX.Element;
    let updatedAt: JSX.Element | null;
    let buttons: JSX.Element;
    let employerName: JSX.Element;
    let primaryLang: JSX.Element;
    let secondaryLang: JSX.Element | null;
    let industry: JSX.Element;
    let salary: JSX.Element;

    if (readOnly) {
      title = <div className="title-font">{jobPostObject?.title}</div>;
      companyName = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {jobPostObject?.companyName}
        </div>
      );
      isRemoteOrCountry = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {jobPostObject?.isRemote ? "Remote" : jobPostObject?.countryName}
        </div>
      );
      updatedAt = (
        <div className="small-font job-full-view-subtitle-item-primary">
          {jobPostObject?.updatedAt}
        </div>
      );
      buttons = (
        <>
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
        </>
      );
      employerName = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Contact ${jobPostObject?.employerName}`}
        </div>
      );
      primaryLang = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Primary Language ${jobPostObject?.primaryLangName}`}
        </div>
      );
      secondaryLang =
        jobPostObject?.secondaryLangName &&
        jobPostObject?.secondaryLangName != jobPostObject?.primaryLangName ? (
          <div className="job-full-view-subtitle-item-secondary">
            {`Secondary Language ${jobPostObject?.secondaryLangName}`}
          </div>
        ) : null;
      industry = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Industry ${jobPostObject?.industryName}`}
        </div>
      );
      salary = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Base Salary ${jobPostObject?.salary}`}
        </div>
      );
    } else {
      title = (
        <div className="left-align">
          <label htmlFor="job-title-input" style={{ marginRight: "1em" }}>
            Title
          </label>
          <input
            id="job-title-input"
            type="text"
            value={jobPostObject?.title}
            onChange={onChangeTitle}
            className="input normal-font"
            name="title"
            style={{ width: "20em", textAlign: "right" }}
          />
        </div>
      );
      companyName = (
        <DropDown
          key={`dd-${uuidv4()}`}
          label="Company"
          name="companyName"
          value={jobPostObject?.companyId}
          onChange={onChangeCompany}
          optionItems={jobPostOptions?.companies || []}
        />
      );
      isRemoteOrCountry = (
        <>
          <div className="sub-title-font job-full-view-subtitle-item-primary">
            <Checkbox
              isChecked={jobPostObject?.isRemote || false}
              toggleIsChecked={toggleIsRemote}
              name="isRemote"
            >
              Remote
            </Checkbox>
          </div>
          {!jobPostObject?.isRemote ? (
            <DropDown
              key={`dd-${uuidv4()}`}
              label="Country"
              name="countryId"
              onChange={onChangeCountry}
              optionItems={jobPostOptions?.countries || []}
            />
          ) : null}
        </>
      );
      updatedAt = null;
      buttons = (
        <>
          <button
            className="primary-btn small-btn"
            style={{ marginBottom: ".5em" }}
            name="save"
            onClick={onClickSubmit}
          >
            save
          </button>
          <button className="secondary-btn small-btn">cancel</button>
        </>
      );
      employerName = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Contact ${jobPostObject?.employerName}`}
        </div>
      );
      primaryLang = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Primary Lang"
            optionItems={jobPostOptions?.languages || []}
            name="primaryLangId"
            onChange={onChangePrimaryLang}
            value={jobPostObject?.primaryLangId}
          />
        </div>
      );
      secondaryLang = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Secondary Lang"
            optionItems={jobPostOptions?.languages || []}
            name="secondaryLangId"
            onChange={onChangeSecondaryLang}
            value={jobPostObject?.secondaryLangId}
          />
        </div>
      );
      industry = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Industry"
            optionItems={jobPostOptions?.industries || []}
            name="industryId"
            onChange={onChangeIndustry}
            value={jobPostObject?.industryId}
          />
        </div>
      );
      salary = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Salary"
            optionItems={jobPostOptions?.salaries || []}
            name="salaryId"
            onChange={onChangeSalary}
            value={jobPostObject?.salaryId}
          />
        </div>
      );
    }

    return {
      title,
      companyName,
      isRemoteOrCountry,
      updatedAt,
      buttons,
      employerName,
      primaryLang,
      secondaryLang,
      industry,
      salary,
    };
  };

  const toggleIsRemote = () => {
    const newJobPost: JobPost = {
      ...jobPost,
      isRemote: !jobPost.isRemote,
    };
    setJobPost(newJobPost);
  };

  const onChangeTitle = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    const newJobPost: JobPost = {
      ...jobPost,
      title: e.target.value,
    };
    setJobPost(newJobPost);
  };

  const onChangeCompany = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    const newJobPost: JobPost = {
      ...jobPost,
      companyId: e.target.value,
    };
    setJobPost(newJobPost);
  };

  const onChangeCountry = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    const newJobPost: JobPost = {
      ...jobPost,
      countryId: e.target.value,
    };
    setJobPost(newJobPost);
  };

  const onChangePrimaryLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    const newJobPost: JobPost = {
      ...jobPost,
      primaryLangId: e.target.value,
    };
    setJobPost(newJobPost);
  };

  const onChangeSecondaryLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    const newJobPost: JobPost = {
      ...jobPost,
      secondaryLangId: e.target.value,
    };
    setJobPost(newJobPost);
  };

  const onChangeIndustry = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    const newJobPost: JobPost = {
      ...jobPost,
      industryId: e.target.value,
    };
    setJobPost(newJobPost);
  };

  const onChangeSalary = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    const newJobPost: JobPost = {
      ...jobPost,
      salaryId: e.target.value,
    };
    setJobPost(newJobPost);
  };

  const onClickSubmit = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    console.log("jobPost to submit", jobPost);
  };

  return (
    <form className="userhome-main" style={{ margin: "auto" }}>
      <div
        className="header-container job-full-view-header"
        style={{
          paddingTop: "2em",
          paddingLeft: "2em",
          paddingRight: "2em",
        }}
      >
        <GoBack
          label={
            devOrEmp === DevOrEmployer.Developer
              ? "developer home"
              : "employer home"
          }
        />
      </div>
      <div
        className="opposites"
        style={{
          paddingTop: "2em",
          paddingLeft: "2em",
          paddingRight: "2em",
          marginBottom: "1.5em",
        }}
      >
        <div className="stack">
          {jobPostDisplayComponents?.title}

          <div className="left-align">
            <div className="opposites">
              <div
                className="job-full-view-subtitle"
                style={{
                  width: "100%",
                  alignItems: readOnly ? "center" : "flex-end",
                }}
              >
                {jobPostDisplayComponents?.companyName}
                {jobPostDisplayComponents?.isRemoteOrCountry}
                {jobPostDisplayComponents?.updatedAt}
              </div>
            </div>
          </div>
        </div>

        <div
          className="stack"
          style={{ alignItems: "flex-end", textAlign: "right" }}
        >
          {jobPostDisplayComponents?.buttons}
        </div>
      </div>

      <div
        className="job-full-view-section"
        style={{
          padding: "1.5em",
          marginBottom: "1em",
        }}
      >
        {jobPostDisplayComponents?.employerName}
        {jobPostDisplayComponents?.primaryLang}
        {jobPostDisplayComponents?.secondaryLang}
        {jobPostDisplayComponents?.industry}
        {jobPostDisplayComponents?.salary}
      </div>

      <div
        className="normal-font dev-post-preview-container job-full-view-section"
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
              children: [{ text: "" }],
            },
          ]}
          readOnly={readOnly}
        />
      </div>
    </form>
  );
}
