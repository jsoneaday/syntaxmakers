import { useLocation } from "react-router-dom";
import JobPost from "../../models/JobPost";
import { ChangeEvent, useEffect, useState } from "react";
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

interface JobFullviewProps {
  readOnly: boolean;
}

export default function JobFullview({ readOnly }: JobFullviewProps) {
  const { state } = useLocation();
  const [jobPost, setJobPost] = useState<JobPost>();
  const [salary, setSalary] = useState("");
  const [title, _setTitle] = useState("");
  const [companies, setCompanies] = useState<OptionType[]>([]);
  const [countries, setCountries] = useState<OptionType[]>([]);
  const [languages, setLanguages] = useState<OptionType[]>([]);
  const [industries, setIndustries] = useState<OptionType[]>([]);
  const [salaries, setSalaries] = useState<OptionType[]>([]);
  const [isRemote, setIsRemote] = useState(false);
  const [devOrEmp] = useDevOrEmployer();

  useEffect(() => {
    const currentJobPost = state as JobPost;
    setJobPost(currentJobPost);
    setSalary(appendPlusLargeCurrency(currentJobPost?.salary || ""));
  }, [state]);

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
          console.log("salaries", salaries);
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

  useEffect(() => {
    setIsRemote(jobPost?.isRemote || false);
  }, [jobPost?.isRemote]);

  const onChangeTitle = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
  };

  const toggleIsRemote = () => {
    setIsRemote(!isRemote);
  };

  return (
    <form className="userhome-main" style={{ margin: "auto" }}>
      <div
        style={{ paddingTop: "2em", paddingLeft: "2em", paddingRight: "2em" }}
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
        }}
      >
        <div className="stack">
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

          <div className="left-align">
            <div className="opposites">
              <div
                className="job-full-view-subtitle"
                style={{
                  width: "100%",
                  alignItems: readOnly ? "center" : "flex-end",
                }}
              >
                {readOnly ? (
                  <div className="sub-title-font job-full-view-subtitle-item-primary">
                    {jobPost?.companyName}
                  </div>
                ) : (
                  <DropDown
                    key={`dd-${uuidv4()}`}
                    label="Company"
                    optionItems={companies}
                  />
                )}
                {readOnly ? (
                  <div className="sub-title-font job-full-view-subtitle-item-primary">
                    {jobPost?.isRemote ? "Remote" : jobPost?.countryName}
                  </div>
                ) : isRemote ? (
                  <div className="sub-title-font job-full-view-subtitle-item-primary">
                    <Checkbox
                      isChecked={isRemote}
                      toggleIsChecked={toggleIsRemote}
                    >
                      {jobPost?.isRemote ? "Remote" : jobPost?.countryName}
                    </Checkbox>
                  </div>
                ) : (
                  <DropDown
                    key={`dd-${uuidv4()}`}
                    label="Country"
                    optionItems={countries}
                  />
                )}
                <div className="small-font job-full-view-subtitle-item-primary">
                  {jobPost?.updatedAt}
                </div>
              </div>
            </div>
          </div>
        </div>

        <div
          className="stack"
          style={{ alignItems: "flex-end", textAlign: "right" }}
        >
          {readOnly ? (
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
          ) : (
            <>
              <button
                className="primary-btn small-btn"
                style={{ marginBottom: ".5em" }}
              >
                save
              </button>
              <button className="secondary-btn small-btn">cancel</button>
            </>
          )}
        </div>
      </div>

      <div
        style={{
          paddingLeft: "2em",
          paddingRight: "2em",
        }}
      >
        <div className="job-full-view-subtitle-item-secondary">
          {`Contact ${jobPost?.employerName}`}
        </div>
        {readOnly ? (
          <div className="job-full-view-subtitle-item-secondary">
            {`Primary Language ${jobPost?.primaryLangName}`}
          </div>
        ) : (
          <div style={{ marginTop: ".75em" }}>
            <DropDown
              key={`dd-${uuidv4()}`}
              label="Primary Lang"
              optionItems={languages}
            />
          </div>
        )}
        {readOnly ? (
          jobPost?.secondaryLangName &&
          jobPost?.secondaryLangName != jobPost?.primaryLangName ? (
            <div className="job-full-view-subtitle-item-secondary">
              {`Secondary Language ${jobPost?.secondaryLangName}`}
            </div>
          ) : null
        ) : (
          <div style={{ marginTop: ".75em" }}>
            <DropDown
              key={`dd-${uuidv4()}`}
              label="Secondary Lang"
              optionItems={languages}
            />
          </div>
        )}
        {readOnly ? (
          <div className="job-full-view-subtitle-item-secondary">
            {`Industry ${jobPost?.industryName}`}
          </div>
        ) : (
          <div style={{ marginTop: ".75em" }}>
            <DropDown
              key={`dd-${uuidv4()}`}
              label="Industry"
              optionItems={industries}
            />
          </div>
        )}
        {readOnly ? (
          <div className="job-full-view-subtitle-item-secondary">
            {`Base Salary ${salary}`}
          </div>
        ) : (
          <div style={{ marginTop: ".75em" }}>
            <DropDown
              key={`dd-${uuidv4()}`}
              label="Salary"
              optionItems={salaries}
            />
          </div>
        )}
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
