import { useLocation, useNavigate } from "react-router-dom";
import JobPost from "../../models/JobPost";
import {
  ChangeEvent,
  useEffect,
  useReducer,
  useRef,
  useState,
  MouseEvent,
} from "react";
import "../../theme/job_full_view.css";
import similar from "../../theme/assets/similar.png";
import GoBack from "../navigation/GoBack";
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
  insertJobPost,
  updateJobPost,
} from "../../../domain/repository/JobRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { formatDistanceToNow } from "date-fns";
import { useInTextEditMode } from "../../common/redux/inTextEditMode/InTextEditModeHooks";
import { MarkdownEditor } from "../textEditor/MarkdownEditor";
import { MDXEditorMethods } from "@mdxeditor/editor";
import { useLoginOpen } from "../../common/redux/loginOpen/LoginOpenHooks";
import { PrimaryButton } from "../controls/Buttons";
import { applyJob } from "../../../domain/repository/JobApplicationRepo";
import { Popup } from "../controls/Popup";

type JobPostDisplayComponents = {
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

interface FormState {
  id: number;
  updatedAt: string;
  title: string;
  description: string | undefined | null;
  employerId: number;
  employerName: string;
  companyId: number;
  companyName: string;
  isRemote: boolean;
  primaryLangId: number;
  primaryLangName: string;
  secondaryLangId: number;
  secondaryLangName: string;
  industryId: number;
  industryName: String;
  salaryId: number;
  salary: string;
  companyLogo?: Blob;
  countryId?: number;
  countryName?: string;
}

enum FormActionTypes {
  Id = "id",
  UpdatedAt = "updatedAt",
  EmployerId = "employerId",
  EmployerName = "employerName",
  Title = "title",
  Desc = "desc",
  IsRemote = "isRemote",
  CountryId = "countryId",
  CountryName = "countryName",
  CompanyId = "companyId",
  CompanyName = "companyName",
  CompanyLogo = "companyLogo",
  IndustryId = "industryId",
  IndustryName = "industryName",
  SalaryId = "salaryId",
  Salary = "salary",
  PrimaryLangId = "primaryLangId",
  PrimaryLangName = "primaryLangName",
  SecondaryLangId = "secondaryLangId",
  SecondaryLangName = "secondaryLangName",
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
    case FormActionTypes.EmployerName:
      newState.employerName = action.payload;
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
    case FormActionTypes.CountryName:
      newState.countryName = action.payload;
      break;
    case FormActionTypes.CompanyId:
      newState.companyId = action.payload;
      break;
    case FormActionTypes.CompanyName:
      newState.companyName = action.payload;
      break;
    case FormActionTypes.CompanyLogo:
      newState.companyLogo = action.payload;
      break;
    case FormActionTypes.IndustryId:
      newState.industryId = action.payload;
      break;
    case FormActionTypes.IndustryName:
      newState.industryName = action.payload;
      break;
    case FormActionTypes.SalaryId:
      newState.salaryId = action.payload;
      break;
    case FormActionTypes.Salary:
      newState.salary = action.payload;
      break;
    case FormActionTypes.PrimaryLangId:
      newState.primaryLangId = action.payload;
      break;
    case FormActionTypes.PrimaryLangName:
      newState.primaryLangName = action.payload;
      break;
    case FormActionTypes.SecondaryLangId:
      newState.secondaryLangId = action.payload;
      break;
    case FormActionTypes.SecondaryLangName:
      newState.secondaryLangName = action.payload;
      break;
    default:
      throw new Error(`Action type, ${action.type}, not found`);
  }
  return newState;
}

type Reducer<S, A> = (prevState: S, action: A) => S;

const ApplicationSuccessMsg = "Your application has been sent";
const ApplicationFailedMsg = "Your application attempt has failed";

interface JobFullviewProps {
  readOnly: boolean;
}

export default function JobFullview({ readOnly }: JobFullviewProps) {
  const [applicationMsg, setApplicationMsg] = useState(ApplicationSuccessMsg);
  const [isPopupOpen, setIsPopupOpen] = useState(false);
  const toggleIsPopupOpen = () => setIsPopupOpen(!isPopupOpen);
  const mdRef = useRef<MDXEditorMethods>(null);
  const [profile, _] = useProfile();
  const { state: routeJobPost } = useLocation();
  const navigate = useNavigate();
  const [jobPostDisplayComponents, setJobPostDisplayComponents] =
    useState<JobPostDisplayComponents>();
  const [_inTextEditMode, setInTextEditMode] = useInTextEditMode();
  const [loginOpen, setLoginOpen] = useLoginOpen();
  /// currerntJobPost is used for component state
  const [currentJobPost, setCurrentJobPost] = useReducer<
    Reducer<FormState, FormAction>
  >(reducer, {
    id: 0,
    updatedAt: formatDistanceToNow(new Date()),
    title: "",
    description: null,
    employerId: 1,
    employerName: "",
    isRemote: false,
    companyId: 1,
    companyName: "",
    countryId: 1,
    countryName: "",
    primaryLangId: 1,
    primaryLangName: "",
    secondaryLangId: 1,
    secondaryLangName: "",
    industryId: 1,
    industryName: "",
    salaryId: 1,
    salary: "",
    companyLogo: undefined,
  });
  const [lastCountryId, setLastCountryId] = useState<number>();
  /// formValues used for form submission
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
  const [submitDisabled, setSubmitDisabled] = useState(true);

  useEffect(() => {
    let currentJobPost: JobPost | undefined = undefined;
    if (routeJobPost) {
      currentJobPost = routeJobPost as JobPost;
      setJobPostStates(currentJobPost);
    }
  }, [routeJobPost]);

  useEffect(() => {
    // if there's an employer logged in and no route state, assume new job post
    if (devOrEmp === DevOrEmployer.Employer && profile && !routeJobPost) {
      setCurrentJobPost({
        type: FormActionTypes.EmployerId,
        payload: profile.id,
      });
      setCurrentJobPost({
        type: FormActionTypes.Desc,
        payload: "",
      });
    }
  }, [profile]);

  useEffect(() => {
    if (!readOnly) {
      getJobPostOptions().then((jobPostOptions) => {
        const jobPostDisplayComponentItems =
          getJobPostDisplayComponents(jobPostOptions);
        setJobPostDisplayComponents(jobPostDisplayComponentItems);
      });
    } else {
      const jobPostDisplayComponentItems =
        getJobPostDisplayComponents(undefined);
      setJobPostDisplayComponents(jobPostDisplayComponentItems);
    }
  }, [currentJobPost, profile]);

  const onJobApply = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    // show login or register if not logged in
    if (!profile || !profile.accessToken) {
      console.log("access token is empty");
      setLoginOpen(!loginOpen);
      return;
    }
    // email employer of application
    // update db that user applied
    try {
      await applyJob(
        Number(currentJobPost.id),
        Number(profile.id),
        profile.accessToken
      );
      setApplicationMsg(ApplicationSuccessMsg);
    } catch (e) {
      setApplicationMsg(ApplicationFailedMsg);
    } finally {
      toggleIsPopupOpen();
    }
  };

  const setJobPostStates = (jobPost: JobPost) => {
    setCurrentJobPost({ type: FormActionTypes.Id, payload: jobPost.id });
    setCurrentJobPost({
      type: FormActionTypes.UpdatedAt,
      payload: jobPost.updatedAt,
    });
    setCurrentJobPost({ type: FormActionTypes.Title, payload: jobPost.title });
    setCurrentJobPost({
      type: FormActionTypes.Desc,
      payload: jobPost.description,
    });
    setCurrentJobPost({
      type: FormActionTypes.EmployerId,
      payload: jobPost.employerId,
    });
    setCurrentJobPost({
      type: FormActionTypes.EmployerName,
      payload: jobPost.employerName,
    });
    setCurrentJobPost({
      type: FormActionTypes.IsRemote,
      payload: jobPost.isRemote,
    });
    setCurrentJobPost({
      type: FormActionTypes.CompanyLogo,
      payload: jobPost.companyLogo,
    });
    setCurrentJobPost({
      type: FormActionTypes.CompanyId,
      payload: jobPost.companyId,
    });
    setCurrentJobPost({
      type: FormActionTypes.CompanyName,
      payload: jobPost.companyName,
    });
    setCurrentJobPost({
      type: FormActionTypes.CountryId,
      payload: jobPost.countryId,
    });
    // if no country selected just default to first on list, as there must be at least one
    setLastCountryId(jobPost.countryId || 1);
    setCurrentJobPost({
      type: FormActionTypes.CountryName,
      payload: jobPost.countryName,
    });
    setCurrentJobPost({
      type: FormActionTypes.PrimaryLangId,
      payload: jobPost.primaryLangId,
    });
    setCurrentJobPost({
      type: FormActionTypes.PrimaryLangName,
      payload: jobPost.primaryLangName,
    });
    setCurrentJobPost({
      type: FormActionTypes.SecondaryLangId,
      payload: jobPost.secondaryLangId,
    });
    setCurrentJobPost({
      type: FormActionTypes.SecondaryLangName,
      payload: jobPost.secondaryLangName,
    });
    setCurrentJobPost({
      type: FormActionTypes.IndustryId,
      payload: jobPost.industryId,
    });
    setCurrentJobPost({
      type: FormActionTypes.IndustryName,
      payload: jobPost.industryName,
    });
    setCurrentJobPost({
      type: FormActionTypes.SalaryId,
      payload: jobPost.salaryId,
    });
    setCurrentJobPost({
      type: FormActionTypes.Salary,
      payload: jobPost.salary,
    });
  };

  const getJobPostDisplayComponents = (
    jobPostOptions: JobPostOptions | undefined
  ) => {
    let _title: JSX.Element;
    let _companyName: JSX.Element;
    let _isRemoteOrCountry: JSX.Element;
    let _updatedAt: JSX.Element | null;
    let _buttons: JSX.Element;
    let _employerName: JSX.Element;
    let _primaryLang: JSX.Element;
    let _secondaryLang: JSX.Element | null;
    let _industry: JSX.Element;
    let _salary: JSX.Element;

    _employerName = (
      <div className="job-full-view-subtitle-item-secondary">
        {`Contact ${currentJobPost.employerName}`}
      </div>
    );

    _companyName = (
      <div className="sub-title-font job-full-view-subtitle-item-primary">
        {currentJobPost.companyName}
      </div>
    );

    if (readOnly) {
      _title = <div className="title-font">{currentJobPost.title}</div>;
      _isRemoteOrCountry = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {currentJobPost.isRemote ? "Remote" : currentJobPost.countryName}
        </div>
      );
      _updatedAt = (
        <div className="small-font job-full-view-subtitle-item-primary">
          {currentJobPost.updatedAt}
        </div>
      );
      _buttons = (
        <>
          <PrimaryButton
            containerStyle={{ marginBottom: ".5em" }}
            onClick={onJobApply}
            disabled={!profile ? true : false}
          >
            apply
          </PrimaryButton>

          <img
            src={similar}
            className="job-icon"
            style={{ marginTop: ".50em" }}
            title="similar jobs"
          />
        </>
      );

      _primaryLang = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Primary Language ${currentJobPost.primaryLangName}`}
        </div>
      );
      _secondaryLang =
        currentJobPost.secondaryLangName &&
        currentJobPost.secondaryLangName != currentJobPost.primaryLangName ? (
          <div className="job-full-view-subtitle-item-secondary">
            {`Secondary Language ${currentJobPost.secondaryLangName}`}
          </div>
        ) : null;
      _industry = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Industry ${currentJobPost.industryName}`}
        </div>
      );
      _salary = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Base Salary ${currentJobPost.salary}`}
        </div>
      );
    } else {
      _title = (
        <div className="left-align">
          <label htmlFor="job-title-input" style={{ marginRight: "1em" }}>
            Title
          </label>
          <input
            id="job-title-input"
            type="text"
            value={currentJobPost.title}
            onChange={onChangeTitle}
            className="input normal-font"
            name="title"
            style={{ width: "20em", textAlign: "right" }}
          />
        </div>
      );
      _isRemoteOrCountry = (
        <>
          <div
            className="sub-title-font job-full-view-subtitle-item-primary"
            style={{ marginBottom: ".5em" }}
          >
            <Checkbox
              isChecked={currentJobPost.isRemote || false}
              toggleIsChecked={toggleIsRemote}
              name="isRemote"
            >
              Remote
            </Checkbox>
          </div>
          {!currentJobPost.isRemote ? (
            <DropDown
              keyName={`dd-country-id`}
              label="Country"
              name="countryId"
              onChange={onChangeCountry}
              value={currentJobPost.countryId || ""}
              optionItems={jobPostOptions?.countries || []}
            />
          ) : null}
        </>
      );
      _updatedAt = null;
      _buttons = (
        <>
          <button
            className="primary-btn small-btn"
            style={{
              marginBottom: ".5em",
              cursor: submitDisabled ? "not-allowed" : "pointer",
            }}
            name="save"
            onClick={onClickSubmit}
            disabled={submitDisabled}
          >
            save
          </button>
          <button
            className="secondary-btn small-btn"
            onClick={onClickSaveCancel}
          >
            cancel
          </button>
        </>
      );
      _primaryLang = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            keyName={`dd-primary-lang`}
            label="Primary Lang"
            optionItems={jobPostOptions?.languages || []}
            name="primaryLangId"
            onChange={onChangePrimaryLang}
            value={currentJobPost.primaryLangId}
          />
        </div>
      );
      _secondaryLang = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            keyName={`dd-secondary-lang`}
            label="Secondary Lang"
            optionItems={jobPostOptions?.languages || []}
            name="secondaryLangId"
            onChange={onChangeSecondaryLang}
            value={currentJobPost.secondaryLangId}
          />
        </div>
      );
      _industry = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            keyName={`dd-industry-id`}
            label="Industry"
            optionItems={jobPostOptions?.industries || []}
            name="industryId"
            onChange={onChangeIndustry}
            value={currentJobPost.industryId}
          />
        </div>
      );
      _salary = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            keyName={`dd-salary-id`}
            label="Salary"
            optionItems={jobPostOptions?.salaries || []}
            name="salaryId"
            onChange={onChangeSalary}
            value={currentJobPost.salaryId}
          />
        </div>
      );
    }

    return {
      title: _title,
      companyName: _companyName,
      isRemoteOrCountry: _isRemoteOrCountry,
      updatedAt: _updatedAt,
      buttons: _buttons,
      employerName: _employerName,
      primaryLang: _primaryLang,
      secondaryLang: _secondaryLang,
      industry: _industry,
      salary: _salary,
    };
  };

  const onClickSaveCancel = () => {
    setInTextEditMode(false);
    navigate(-1);
  };

  const toggleIsRemote = () => {
    const toggledIsRemote = !currentJobPost.isRemote;
    setCurrentJobPost({
      type: FormActionTypes.IsRemote,
      payload: toggledIsRemote,
    });

    if (toggledIsRemote) {
      setCurrentJobPost({
        type: FormActionTypes.CountryId,
        payload: undefined,
      });
    } else {
      setCurrentJobPost({
        type: FormActionTypes.CountryId,
        payload: lastCountryId,
      });
    }

    setSubmitDisabled(false);
    setInTextEditMode(true);
  };

  const onChangeTitle = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    setCurrentJobPost({
      type: FormActionTypes.Title,
      payload: e.target.value,
    });

    setSubmitDisabled(false);
    setInTextEditMode(true);
  };

  const onChangeCountry = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();
    const payload = e.target.value ? Number(e.target.value) : 0;
    setCurrentJobPost({
      type: FormActionTypes.CountryId,
      payload,
    });

    setSubmitDisabled(false);
    setLastCountryId(payload);
    setInTextEditMode(true);
  };

  const onChangePrimaryLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    setCurrentJobPost({
      type: FormActionTypes.PrimaryLangId,
      payload: e.target.value ? Number(e.target.value) : 0,
    });

    setSubmitDisabled(false);
    setInTextEditMode(true);
  };

  const onChangeSecondaryLang = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    setCurrentJobPost({
      type: FormActionTypes.SecondaryLangId,
      payload: e.target.value ? Number(e.target.value) : 0,
    });

    setSubmitDisabled(false);
    setInTextEditMode(true);
  };

  const onChangeIndustry = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    setCurrentJobPost({
      type: FormActionTypes.IndustryId,
      payload: e.target.value ? Number(e.target.value) : 0,
    });

    setSubmitDisabled(false);
    setInTextEditMode(true);
  };

  const onChangeSalary = (e: React.ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    setCurrentJobPost({
      type: FormActionTypes.SalaryId,
      payload: e.target.value ? Number(e.target.value) : 0,
    });

    setSubmitDisabled(false);
    setInTextEditMode(true);
  };

  const onClickSubmit = async (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    try {
      setSubmitDisabled(true);
      setFormValues();

      if (!profile || !profile.accessToken) {
        throw new Error(
          `Access token is required to save a job record ${profile}`
        );
      }

      if (formValues.current.id === 0) {
        await insertJobPost(formValues.current, profile.accessToken);
      } else {
        await updateJobPost(formValues.current, profile.accessToken);
      }
    } catch (e) {
      console.log(e);
    } finally {
      const state = refreshUrlState();
      navigate(".", { state, replace: true });
      setSubmitDisabled(false);
      setInTextEditMode(false);
    }
  };

  const refreshUrlState = () => {
    const state: JobPost = {
      key: routeJobPost ? routeJobPost.key : uuidv4(),
      id: currentJobPost.id,
      updatedAt: currentJobPost.updatedAt,
      employerId: currentJobPost.employerId,
      employerName: currentJobPost.employerName,
      title: currentJobPost.title,
      description: currentJobPost.description
        ? JSON.stringify(currentJobPost.description)
        : "",
      isRemote: currentJobPost.isRemote,
      companyId: currentJobPost.companyId,
      companyName: currentJobPost.companyName,
      companyLogo: currentJobPost.companyLogo,
      primaryLangId: currentJobPost.primaryLangId,
      primaryLangName: currentJobPost.primaryLangName,
      secondaryLangId: currentJobPost.secondaryLangId,
      secondaryLangName: currentJobPost.secondaryLangName,
      industryId: currentJobPost.industryId,
      industryName: currentJobPost.industryName,
      salaryId: currentJobPost.salaryId,
      salary: currentJobPost.salary,
      countryId: currentJobPost.countryId,
      countryName: currentJobPost.countryName,
    };
    return state;
  };

  const setFormValues = () => {
    formValues.current = {
      id: currentJobPost.id,
      employerId: currentJobPost.employerId,
      title: currentJobPost.title,
      description: JSON.stringify(currentJobPost.description || ""),
      isRemote: currentJobPost.isRemote,
      primaryLangId: currentJobPost.primaryLangId,
      secondaryLangId: currentJobPost.secondaryLangId,
      industryId: currentJobPost.industryId,
      salaryId: currentJobPost.salaryId,
      countryId: currentJobPost.isRemote ? undefined : currentJobPost.countryId,
    };
  };

  return (
    <>
      <Popup isOpen={isPopupOpen} toggleOpen={toggleIsPopupOpen}>
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            justifyContent: "center",
            padding: "0.5em",
            borderBottom: "solid 1px var(--border-cl)",
          }}
        >
          {applicationMsg}
        </div>
      </Popup>
      <form className="userhome-main" style={{ margin: "auto" }}>
        <div className="header-container job-full-view-header">
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
                  }}
                >
                  <div style={{ marginBottom: ".5em" }}>
                    {jobPostDisplayComponents?.companyName}
                  </div>
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
          {currentJobPost.description ? (
            <MarkdownEditor
              mdRef={mdRef}
              readOnly={readOnly}
              markdown={currentJobPost.description}
            />
          ) : null}
        </div>
      </form>
    </>
  );
}
