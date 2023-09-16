import { useLocation } from "react-router-dom";
import JobPost from "../../models/JobPost";
import {
  ChangeEvent,
  useEffect,
  useReducer,
  useState,
  useTransition,
} from "react";
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
import { JobPostData, getJobPostData } from "../../models/JobFullviewModel";

interface FormState {
  id: string;
  updatedAt: string;
  employerId: string;
  employerFullName: string;
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

enum FormActionTypes {
  Id = "id",
  UpdatedAt = "updatedAt",
  EmployerId = "employerId",
  EmployerFullName = "employerFullName",
  Title = "title",
  Desc = "desc",
  IsRemote = "isRemote",
  CountryId = "countryId",
  CompanyId = "companyId",
  IndustryId = "industryId",
  SalaryId = "salaryId",
  PrimaryLangId = "primaryLangId",
  SecondaryLangId = "secondaryLangId",
}

interface FormAction {
  type: FormActionTypes;
  payload: any;
}

function reducer(state: FormState, action: FormAction): FormState {
  const newState = { ...state };

  switch (action.type) {
    case FormActionTypes.Id:
      newState.id = action.payload;
      break;
    case FormActionTypes.UpdatedAt:
      newState.updatedAt = action.payload;
      break;
    case FormActionTypes.EmployerId:
      newState.employerId = action.payload;
      break;
    case FormActionTypes.EmployerFullName:
      newState.employerFullName = action.payload;
      break;
    case FormActionTypes.Title:
      newState.title = action.payload;
      break;
    case FormActionTypes.Desc:
      newState.description = action.payload;
      break;
    case FormActionTypes.IsRemote:
      newState.isRemote = action.payload;
      break;
    case FormActionTypes.CountryId:
      newState.countryId = action.payload;
      break;
    case FormActionTypes.CompanyId:
      newState.companyId = action.payload;
      break;
    case FormActionTypes.IndustryId:
      newState.industryId = action.payload;
      break;
    case FormActionTypes.SalaryId:
      newState.salaryId = action.payload;
      break;
    case FormActionTypes.PrimaryLangId:
      newState.primaryLangId = action.payload;
      break;
    case FormActionTypes.SecondaryLangId:
      newState.secondaryLangId = action.payload;
      break;
    default:
      throw new Error(`Action type, ${action.type}, not found`);
  }

  return newState;
}

type Reducer<S, A> = (prevState: S, action: A) => S;

interface JobFullviewProps {
  readOnly: boolean;
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

export default function JobFullview({ readOnly }: JobFullviewProps) {
  const { state: routeJobPost } = useLocation();
  const [jobPostDisplayComponents, setJobPostDisplayComponents] =
    useState<JobPostDisplayObject>();
  const [formValues, setFormValues] = useReducer<
    Reducer<FormState, FormAction>
  >(reducer, {
    id: "",
    updatedAt: "",
    employerId: "",
    employerFullName: "",
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
  const [jobPostData, setJobPostData] = useState<JobPostData>();
  const [devOrEmp] = useDevOrEmployer();
  const [_isPending, startTransition] = useTransition();

  useEffect(() => {
    if (routeJobPost) {
      const currentJobPost = routeJobPost as JobPost;

      const jobPostDisplayComponentItems =
        getJobPostDisplayComponents(currentJobPost);
      setJobPostDisplayComponents(jobPostDisplayComponentItems);

      setAllFormValues(currentJobPost);
    }
  }, [routeJobPost]);

  useEffect(() => {
    if (!readOnly) {
      getJobPostData().then((jobPostData) => {
        setJobPostData(jobPostData);
      });
    }
  }, [readOnly]);

  const setAllFormValues = (jobPost: JobPost) => {
    startTransition(() => {
      setFormValues({ type: FormActionTypes.Id, payload: jobPost.id });
      setFormValues({
        type: FormActionTypes.UpdatedAt,
        payload: jobPost.updatedAt,
      });
      setFormValues({
        type: FormActionTypes.EmployerId,
        payload: jobPost.employerId,
      });
      setFormValues({
        type: FormActionTypes.EmployerFullName,
        payload: jobPost.employerName,
      });
      setFormValues({ type: FormActionTypes.Title, payload: jobPost.title });
      setFormValues({
        type: FormActionTypes.Desc,
        payload: jobPost.description,
      });
      setFormValues({
        type: FormActionTypes.CountryId,
        payload: jobPost.countryId,
      });
      setFormValues({
        type: FormActionTypes.CompanyId,
        payload: jobPost.companyId,
      });
      setFormValues({
        type: FormActionTypes.IndustryId,
        payload: jobPost.industryId,
      });
      setFormValues({
        type: FormActionTypes.SalaryId,
        payload: jobPost.salaryId,
      });
      setFormValues({
        type: FormActionTypes.PrimaryLangId,
        payload: jobPost.primaryLangId,
      });
      setFormValues({
        type: FormActionTypes.SecondaryLangId,
        payload: jobPost.secondaryLangId,
      });
    });
  };

  const onChangeTitle = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    setFormValues({ type: FormActionTypes.Title, payload: e.target.value });
  };

  const getJobPostDisplayComponents = (jobPostObject: JobPost | undefined) => {
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
            value={formValues.title}
            onChange={onChangeTitle}
            className="input"
          />
        </div>
      );
      companyName = (
        <DropDown
          key={`dd-${uuidv4()}`}
          label="Company"
          optionItems={jobPostData?.companies || []}
        />
      );
      isRemoteOrCountry = (
        <>
          <div className="sub-title-font job-full-view-subtitle-item-primary">
            <Checkbox
              isChecked={formValues.isRemote}
              toggleIsChecked={toggleIsRemote}
            >
              Remote
            </Checkbox>
          </div>
          {!formValues.isRemote ? (
            <DropDown
              key={`dd-${uuidv4()}`}
              label="Country"
              optionItems={jobPostData?.countries || []}
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
          >
            save
          </button>
          <button className="secondary-btn small-btn">cancel</button>
        </>
      );
      employerName = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Contact ${formValues.employerFullName}`}
        </div>
      );
      primaryLang = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Primary Lang"
            optionItems={jobPostData?.languages || []}
          />
        </div>
      );
      secondaryLang = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Secondary Lang"
            optionItems={jobPostData?.languages || []}
          />
        </div>
      );
      industry = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Industry"
            optionItems={jobPostData?.industries || []}
          />
        </div>
      );
      salary = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Salary"
            optionItems={jobPostData?.salaries || []}
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
    setFormValues({
      type: FormActionTypes.IsRemote,
      payload: !formValues.isRemote,
    });
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
              children: [{ text: "A line of text in a paragraph." }],
            },
          ]}
          readOnly={readOnly}
        />
      </div>
    </form>
  );
}
