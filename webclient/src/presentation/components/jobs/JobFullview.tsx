import { useLocation } from "react-router-dom";
import JobPost from "../../models/JobPost";
import {
  ChangeEvent,
  useEffect,
  useMemo,
  useReducer,
  useState,
  useTransition,
} from "react";
import "../../theme/job_full_view.css";
import { appendPlusLargeCurrency } from "../../common/CurrencyFormatter";
import flag from "../../theme/assets/flag.png";
import similar from "../../theme/assets/similar.png";
import GoBack from "../../components/navigation/GoBack";
import TextEditor from "../../components/textEditor/TextEditor";
import DropDown, { OptionType } from "../controls/DropDown";
import Checkbox from "../controls/Checkbox";
import { getCountries } from "../../../domain/repository/CountryRepo";
import { convert as convertCountry } from "../../models/CountryModel";
import { getCompanies } from "../../../domain/repository/CompanyRepo";
import { convert as convertCompany } from "../../models/CompanyModel";
import { useDevOrEmployer } from "../../common/redux/devOrEmployer/DevOrEmployerHooks";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import { getLanguages } from "../../../domain/repository/LanguageRepo";
import { convert as convertLang } from "../../models/LanguageModel";
import { getIndustries } from "../../../domain/repository/IndustryRepo";
import { convert as convertIndustry } from "../../models/IndustryModel";
import { getSalaries } from "../../../domain/repository/SalaryRepo";
import { convert as convertSalary } from "../../models/SalaryModel";
/// @ts-ignore
import { v4 as uuidv4 } from "uuid";

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

export default function JobFullview({ readOnly }: JobFullviewProps) {
  const { state: routeJobPost } = useLocation();
  const [readOnlyJobPost, setReadonlyJobPost] = useState<JobPost | null>(null);
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
  const [companies, setCompanies] = useState<OptionType[]>([]);
  const [countries, setCountries] = useState<OptionType[]>([]);
  const [languages, setLanguages] = useState<OptionType[]>([]);
  const [industries, setIndustries] = useState<OptionType[]>([]);
  const [salaries, setSalaries] = useState<OptionType[]>([]);
  const [devOrEmp] = useDevOrEmployer();
  const [_isPending, startTransition] = useTransition();

  useEffect(() => {
    const currentJobPost = routeJobPost as JobPost;
    setReadonlyJobPost(currentJobPost);
    setAllFormValues(currentJobPost);
  }, [routeJobPost]);

  useEffect(() => {
    if (!readOnly) {
      getCompanies()
        .then((companies) => {
          setCompanies(
            companies.map((company) => {
              const model = convertCompany(company);
              return { name: model.name, value: model.id };
            })
          );
        })
        .catch((err) => console.log("Failed to get companies", err));

      getCountries()
        .then((countries) => {
          setCountries(
            countries.map((country) => {
              const model = convertCountry(country);
              return { name: model.name, value: model.id };
            })
          );
        })
        .catch((err) => console.log("Failed to get countries", err));

      getLanguages()
        .then((languages) => {
          setLanguages(
            languages.map((language) => {
              const model = convertLang(language);
              return { name: model.name, value: model.id };
            })
          );
        })
        .catch((err) => console.log("Failed to get languages", err));

      getIndustries()
        .then((industries) => {
          setIndustries(
            industries.map((industry) => {
              const model = convertIndustry(industry);
              return { name: model.name, value: model.id };
            })
          );
        })
        .catch((err) => console.log("Failed to get industries", err));

      getSalaries()
        .then((salaries) => {
          setSalaries(
            salaries.map((salary) => {
              const model = convertSalary(salary);

              return {
                name: appendPlusLargeCurrency(model.base),
                value: model.id,
              };
            })
          );
        })
        .catch((err) => console.log("Failed to get salaries", err));
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

  const jobPostDisplayObject = useMemo(() => {
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
      title = <div className="title-font">{readOnlyJobPost?.title}</div>;
      companyName = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {readOnlyJobPost?.companyName}
        </div>
      );
      isRemoteOrCountry = (
        <div className="sub-title-font job-full-view-subtitle-item-primary">
          {readOnlyJobPost?.isRemote ? "Remote" : readOnlyJobPost?.countryName}
        </div>
      );
      updatedAt = (
        <div className="small-font job-full-view-subtitle-item-primary">
          {readOnlyJobPost?.updatedAt}
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
          {`Contact ${readOnlyJobPost?.employerName}`}
        </div>
      );
      primaryLang = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Primary Language ${readOnlyJobPost?.primaryLangName}`}
        </div>
      );
      secondaryLang =
        readOnlyJobPost?.secondaryLangName &&
        readOnlyJobPost?.secondaryLangName !=
          readOnlyJobPost?.primaryLangName ? (
          <div className="job-full-view-subtitle-item-secondary">
            {`Secondary Language ${readOnlyJobPost?.secondaryLangName}`}
          </div>
        ) : null;
      industry = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Industry ${readOnlyJobPost?.industryName}`}
        </div>
      );
      salary = (
        <div className="job-full-view-subtitle-item-secondary">
          {`Base Salary ${readOnlyJobPost?.salary}`}
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
          optionItems={companies}
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
              optionItems={countries}
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
            optionItems={languages}
          />
        </div>
      );
      secondaryLang = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Secondary Lang"
            optionItems={languages}
          />
        </div>
      );
      industry = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Industry"
            optionItems={industries}
          />
        </div>
      );
      salary = (
        <div style={{ marginTop: ".75em" }}>
          <DropDown
            key={`dd-${uuidv4()}`}
            label="Salary"
            optionItems={salaries}
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
  }, [readOnly]);

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
          {jobPostDisplayObject.title}

          <div className="left-align">
            <div className="opposites">
              <div
                className="job-full-view-subtitle"
                style={{
                  width: "100%",
                  alignItems: readOnly ? "center" : "flex-end",
                }}
              >
                {jobPostDisplayObject.companyName}
                {jobPostDisplayObject.isRemoteOrCountry}
                {jobPostDisplayObject.updatedAt}
              </div>
            </div>
          </div>
        </div>

        <div
          className="stack"
          style={{ alignItems: "flex-end", textAlign: "right" }}
        >
          {jobPostDisplayObject.buttons}
        </div>
      </div>

      <div
        className="job-full-view-section"
        style={{
          padding: "1.5em",
          marginBottom: "1em",
        }}
      >
        {jobPostDisplayObject.employerName}
        {jobPostDisplayObject.primaryLang}
        {jobPostDisplayObject.secondaryLang}
        {jobPostDisplayObject.industry}
        {jobPostDisplayObject.salary}
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
