import { useLocation, useNavigate } from "react-router-dom";
import JobPost from "../../models/JobPost";
import { ChangeEvent, useEffect, useRef, useState } from "react";
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
import {
  JobFormState,
  updateJobPost,
} from "../../../domain/repository/JobRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";

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
  const [profile, _] = useProfile();
  const { state: routeJobPost } = useLocation();
  const navigate = useNavigate();
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
  const formValues = useRef<JobFormState>({
    id: 0,
    employerId: 0,
    title: "",
    description: "",
    isRemote: false,
    countryId: 0,
    industryId: 0,
    salaryId: 0,
    primaryLangId: 0,
    secondaryLangId: 0,
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
    } else {
      const jobPostDisplayComponentItems = getJobPostDisplayComponents(
        undefined,
        jobPost
      );
      setJobPostDisplayComponents(jobPostDisplayComponentItems);
    }
  }, [jobPost]);

  const getJobPostDisplayComponents = (
    jobPostOptions: JobPostOptions | undefined,
    jobPostObject: JobPost
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
      title = <div className="title-font">{jobPostObject.title}</div>;
      companyName = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {jobPostObject.companyName}
        </div>
      );
      isRemoteOrCountry = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {jobPostObject.isRemote ? "Remote" : jobPostObject.countryName}
        </div>
      );
      updatedAt = (
        <div className="small-font job-full-view-subtitle-item-primary">
          {jobPostObject.updatedAt}
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
          {`Contact ${jobPostObject.employerName}`}
        </div>
      );
      primaryLang = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Primary Language ${jobPostObject.primaryLangName}`}
        </div>
      );
      secondaryLang =
        jobPostObject.secondaryLangName &&
        jobPostObject.secondaryLangName != jobPostObject.primaryLangName ? (
          <div className="job-full-view-subtitle-item-secondary">
            {`Secondary Language ${jobPostObject.secondaryLangName}`}
          </div>
        ) : null;
      industry = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Industry ${jobPostObject.industryName}`}
        </div>
      );
      salary = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Base Salary ${jobPostObject.salary}`}
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
            value={jobPostObject.title}
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
          value={jobPostObject.companyId}
          onChange={onChangeCompany}
          optionItems={jobPostOptions?.companies || []}
        />
      );
      isRemoteOrCountry = (
        <>
          <div className="sub-title-font job-full-view-subtitle-item-primary">
            <Checkbox
              isChecked={jobPostObject.isRemote || false}
              toggleIsChecked={toggleIsRemote}
              name="isRemote"
            >
              Remote
            </Checkbox>
          </div>
          {!jobPostObject.isRemote ? (
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
          {`Contact ${jobPostObject.employerName}`}
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
            value={jobPostObject.primaryLangId}
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
            value={jobPostObject.secondaryLangId}
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
            value={jobPostObject.industryId}
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
            value={jobPostObject.salaryId}
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
    console.log("title", e.target.value);
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

    console.log("before primaryLangId", jobPost.primaryLangId);
    const newJobPost: JobPost = {
      ...jobPost,
      primaryLangId: e.target.value,
    };
    console.log("after primaryLangId", e.target.value);
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

  const onClickSubmit = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    console.log("jobPost to submit", jobPost);
    setFormValues();
    if (!profile || !profile?.accessToken) {
      throw new Error("Access token is required to save a job record");
    }
    await updateJobPost(formValues.current, profile.accessToken);
    console.log("jobPost after submit", jobPost);
    navigate(".", { state: jobPost }); // need this since route state stays on older value
  };

  const setFormValues = () => {
    formValues.current = {
      id: Number(jobPost.id),
      employerId: Number(jobPost.employerId),
      title: jobPost.title,
      description: jobPost.description,
      isRemote: jobPost.isRemote,
      primaryLangId: Number(jobPost.primaryLangId),
      industryId: Number(jobPost.industryId),
      salaryId: Number(jobPost.salaryId),
      secondaryLangId: Number(jobPost.secondaryLangId),
      countryId: Number(jobPost.countryId),
    };
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
