import { useLocation, useNavigate } from "react-router-dom";
import JobPost from "../../models/JobPost";
import {
  ChangeEvent,
  useEffect,
  useReducer,
  useRef,
  useState,
  FormEvent,
} from "react";
import "../../theme/job_full_view.css";
import GoBack from "../navigation/GoBack";
import DropDown from "../controls/DropDown";
import Checkbox from "../controls/Checkbox";
import { useDevOrEmployer } from "../../common/redux/devOrEmployer/DevOrEmployerHooks";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";
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
import {
  applyJob,
  developerAppliedToJob,
} from "../../../domain/repository/JobApplicationRepo";
import { Popup } from "../controls/Popup";
import { ValidationMsgView } from "../controls/ValidationMsgView";
import EmpProfile from "../../models/EmpProfile";

type JobPostDisplayComponents = {
  title: JSX.Element;
  companyName: JSX.Element;
  isRemoteOrCountry: JSX.Element;
  updatedAt: JSX.Element | null;
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
  userType: UiDevOrEmployer;
}

export default function JobFullview({ userType }: JobFullviewProps) {
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
    employerId: profile ? Number(profile.id) : 0,
    employerName: "",
    isRemote: false,
    companyId: profile ? Number((profile as EmpProfile).companyId) : 0,
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
  const [lastCountryId, setLastCountryId] = useState<number | undefined>(1);
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
  const [submitDisabled, setSubmitDisabled] = useState(false);
  const [validationMessage, setValidationMessage] = useState("");
  const [successMessage, setSuccessMessage] = useState("");

  useEffect(() => {
    let currentJobPost: JobPost | undefined = undefined;
    if (routeJobPost) {
      currentJobPost = routeJobPost as JobPost;
      setJobPostStates(currentJobPost);
    } else {
      setJobPostStates(null);
    }
  }, [routeJobPost]);

  useEffect(() => {
    // if there's an employer logged in and no route state, assume new job post
    if (devOrEmp === UiDevOrEmployer.Employer && profile && !routeJobPost) {
      setCurrentJobPost({
        type: FormActionTypes.EmployerId,
        payload: profile.id,
      });
      setCurrentJobPost({
        type: FormActionTypes.Desc,
        payload: "",
      });
      mdRef.current?.setMarkdown("");
    }
  }, [profile]);

  useEffect(() => {
    if (
      profile &&
      currentJobPost.id > 0 &&
      userType === UiDevOrEmployer.Developer
    ) {
      developerAppliedToJob(currentJobPost.id, Number(profile.id)).then(
        (disableApplyBtn) => {
          setSubmitDisabled(disableApplyBtn);
        }
      );
    }

    if (userType === UiDevOrEmployer.Employer) {
      getJobPostOptions().then((jobPostOptions) => {
        getJobPostDisplayComponents(jobPostOptions).then(
          (jobPostDisplayComponentItems) =>
            setJobPostDisplayComponents(jobPostDisplayComponentItems)
        );
      });
    } else {
      getJobPostDisplayComponents(undefined).then(
        (jobPostDisplayComponentItems) =>
          setJobPostDisplayComponents(jobPostDisplayComponentItems)
      );
    }
  }, [currentJobPost, profile]);

  const getMarkdownText = (markdown: string) => {
    currentJobPost.description = markdown;
  };

  const setJobPostStates = (jobPost: JobPost | null) => {
    setCurrentJobPost({ type: FormActionTypes.Id, payload: jobPost?.id || 0 });
    setCurrentJobPost({
      type: FormActionTypes.UpdatedAt,
      payload: jobPost?.updatedAt || formatDistanceToNow(new Date()),
    });
    setCurrentJobPost({
      type: FormActionTypes.Title,
      payload: jobPost?.title || "",
    });
    setCurrentJobPost({
      type: FormActionTypes.Desc,
      payload: jobPost?.description || null,
    });
    mdRef.current?.setMarkdown(jobPost?.description || "");
    setCurrentJobPost({
      type: FormActionTypes.EmployerId,
      payload: jobPost?.employerId || Number(profile!.id),
    });
    setCurrentJobPost({
      type: FormActionTypes.EmployerName,
      payload: jobPost?.employerName || "",
    });
    setCurrentJobPost({
      type: FormActionTypes.IsRemote,
      payload: jobPost?.isRemote || false,
    });
    setCurrentJobPost({
      type: FormActionTypes.CompanyLogo,
      payload: jobPost?.companyLogo,
    });
    setCurrentJobPost({
      type: FormActionTypes.CompanyId,
      payload: jobPost?.companyId || 1,
    });
    setCurrentJobPost({
      type: FormActionTypes.CompanyName,
      payload: jobPost?.companyName || "",
    });
    setCurrentJobPost({
      type: FormActionTypes.CountryId,
      payload: jobPost?.countryId || 1,
    });
    // if no country selected just default to first on list, as there must be at least one
    setLastCountryId(jobPost?.countryId || 1);
    setCurrentJobPost({
      type: FormActionTypes.CountryName,
      payload: jobPost?.countryName || "",
    });
    setCurrentJobPost({
      type: FormActionTypes.PrimaryLangId,
      payload: jobPost?.primaryLangId || 1,
    });
    setCurrentJobPost({
      type: FormActionTypes.PrimaryLangName,
      payload: jobPost?.primaryLangName || "",
    });
    setCurrentJobPost({
      type: FormActionTypes.SecondaryLangId,
      payload: jobPost?.secondaryLangId || 1,
    });
    setCurrentJobPost({
      type: FormActionTypes.SecondaryLangName,
      payload: jobPost?.secondaryLangName || "",
    });
    setCurrentJobPost({
      type: FormActionTypes.IndustryId,
      payload: jobPost?.industryId || 1,
    });
    setCurrentJobPost({
      type: FormActionTypes.IndustryName,
      payload: jobPost?.industryName || "",
    });
    setCurrentJobPost({
      type: FormActionTypes.SalaryId,
      payload: jobPost?.salaryId || 1,
    });
    setCurrentJobPost({
      type: FormActionTypes.Salary,
      payload: jobPost?.salary || "",
    });
  };

  const getJobPostDisplayComponents = async (
    jobPostOptions: JobPostOptions | undefined
  ) => {
    let _title: JSX.Element;
    let _companyName: JSX.Element;
    let _isRemoteOrCountry: JSX.Element;
    let _updatedAt: JSX.Element | null;
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

    if (userType === UiDevOrEmployer.Developer) {
      _title = <div className="title-font">{currentJobPost.title}</div>;
      _isRemoteOrCountry = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {currentJobPost.isRemote ? "Remote" : currentJobPost.countryName}
        </div>
      );
      _updatedAt = (
        <div
          className="small-font job-full-view-subtitle-item-primary"
          style={{ marginTop: ".5em" }}
        >
          <span style={{ marginRight: ".5em" }}>updated</span>
          {currentJobPost.updatedAt}
        </div>
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
            style={{ width: "20em" }}
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

      _updatedAt = (
        <div
          className="small-font job-full-view-subtitle-item-primary"
          style={{ marginTop: ".5em" }}
        >
          <span style={{ marginRight: ".5em" }}>updated</span>
          {currentJobPost.updatedAt}
        </div>
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
      employerName: _employerName,
      primaryLang: _primaryLang,
      secondaryLang: _secondaryLang,
      industry: _industry,
      salary: _salary,
    };
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

  const onJobSubmit = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (userType === UiDevOrEmployer.Developer) {
      await onJobApply();
    } else {
      await onJobSave();
    }
  };

  const onJobApply = async () => {
    // show login or register if not logged in
    if (!profile || !profile.accessToken) {
      setLoginOpen(!loginOpen);
      return;
    }
    // email employer of application
    // update db that user applied
    try {
      setSubmitDisabled(true); // if apply succeeds do NOT reenable apply button!

      await applyJob(
        Number(currentJobPost.id),
        Number(profile.id),
        profile.accessToken
      );
      setSuccessMessage(ApplicationSuccessMsg);
      setValidationMessage("");
    } catch (e) {
      setSubmitDisabled(false);
      setValidationMessage(ApplicationFailedMsg);
      setSuccessMessage("");
    } finally {
      toggleIsPopupOpen();
    }
  };

  const onJobSave = async () => {
    try {
      setSubmitDisabled(true);
      setFormValues();
      setValidationMessage("");
      setSuccessMessage("");
      if (!validateFormValues()) return;

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

      const state = refreshUrlState();
      navigate(".", { state, replace: true });

      setValidationMessage("");
      setSuccessMessage("Job saved successfully");
    } catch (e) {
      console.log(e);
      if (e instanceof Error) {
        setValidationMessage(e.message);
        setSuccessMessage("");
      } else {
        setValidationMessage("Failed to save job");
        setSuccessMessage("");
      }
    } finally {
      setInTextEditMode(false);
      setSubmitDisabled(false);
      toggleIsPopupOpen();
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
      description: currentJobPost.description || "",
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

  const validateFormValues = () => {
    let result = true;
    if (
      formValues.current.title.length < 3 ||
      formValues.current.title.length > 100
    ) {
      setValidationMessage(
        "Title must be greater than 3 characters and less than 100"
      );
      setSuccessMessage("");
      result = false;
    } else if (
      formValues.current.description.length < 3 ||
      formValues.current.description.length > 8000
    ) {
      setValidationMessage(
        "Description must be greater than 3 characters and less than 8000"
      );
      setSuccessMessage("");
      result = false;
    } else if (formValues.current.isRemote && formValues.current.countryId) {
      setValidationMessage("Country cannot be selected when Remote is checked");
      setSuccessMessage("");
      result = false;
    } else if (!formValues.current.isRemote && !formValues.current.countryId) {
      setValidationMessage("Country cannot be empty when not is remote");
      setSuccessMessage("");
      result = false;
    }
    return result;
  };

  const setFormValues = () => {
    formValues.current = {
      id: currentJobPost.id,
      employerId: currentJobPost.employerId,
      title: currentJobPost.title,
      description: currentJobPost.description || "",
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
        {isPopupOpen ? (
          <div
            style={{
              display: "flex",
              flexDirection: "column",
              justifyContent: "center",
              height: "50px",
            }}
          >
            <ValidationMsgView
              validationMessage={validationMessage}
              successMessage={successMessage}
            />
          </div>
        ) : null}
      </Popup>
      <form
        className="userhome-main"
        style={{ margin: "auto", marginBottom: "2em" }}
        onSubmit={onJobSubmit}
      >
        <div className="header-container job-full-view-header">
          <GoBack
            label={
              devOrEmp === UiDevOrEmployer.Developer
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
            style={{
              alignItems: "flex-end",
              textAlign: "right",
            }}
          >
            {userType === UiDevOrEmployer.Developer ? (
              <PrimaryButton
                type="submit"
                containerStyle={{ marginBottom: ".5em" }}
                disabled={submitDisabled}
              >
                apply
              </PrimaryButton>
            ) : (
              <button
                type="submit"
                className="primary-btn small-btn"
                style={{
                  marginBottom: ".5em",
                  cursor: submitDisabled ? "not-allowed" : "pointer",
                }}
                name="save"
                disabled={submitDisabled}
              >
                Save
              </button>
            )}
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
            width: "100%",
            padding: "2em",
          }}
        >
          <span className="title-font" style={{ marginBottom: "1em" }}>
            Description
          </span>
          <MarkdownEditor
            mdRef={mdRef}
            readOnly={userType === UiDevOrEmployer.Developer}
            markdown={currentJobPost.description || ""}
            getChangedText={getMarkdownText}
          />
        </div>
      </form>
    </>
  );
}
