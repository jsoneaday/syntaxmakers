import { ChangeEvent, FormEvent, useEffect, useRef, useState } from "react";
import { UiDevOrEmployer } from "../../models/DevOrEmployer";
import DropDown, { OptionType } from "../controls/DropDown";
import { getLanguages } from "../../../domain/repository/LanguageRepo";
import {
  createDeveloper,
  getDeveloperByEmail,
  updateDeveloper,
} from "../../../domain/repository/DeveloperRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import DevProfile, { convert as convertDev } from "../../models/DevProfile";
import EmpProfile, { convert as convertEmp } from "../../models/EmpProfile";
import { ValidationMsgView } from "../controls/ValidationMsgView";
import { ChangePassword } from "./ChangePassword";
import { PrimaryButton } from "../controls/Buttons";
import { MarkdownEditor } from "../textEditor/MarkdownEditor";
import { MDXEditorMethods } from "@mdxeditor/editor";
import { ActionMeta, SingleValue, StylesConfig } from "react-select";
import CreateableSelect from "react-select/creatable";
import { getCompanies } from "../../../domain/repository/CompanyRepo";
import {
  createEmployer,
  getEmployerByEmail,
  updateEmployer,
} from "../../../domain/repository/EmployerRepo";

interface SelectOptionType {
  value: string;
  label: string;
}

export enum ProfileFormEditMode {
  Create,
  Edit,
}

interface DevFormData {
  userName: string;
  fullName: string;
  email: string;
  description: string;
  password: string;
  primaryLangId: number;
  secondaryLangId?: number | null;
}

interface EmpFormData {
  userName: string;
  fullName: string;
  email: string;
  password: string;
  companyId: string;
}

interface ProfileFormProps {
  isModalMode: boolean;
  editMode: ProfileFormEditMode;
  userType: UiDevOrEmployer;
}

/// form to register or edit profile
export function ProfileForm({
  isModalMode,
  editMode,
  userType,
}: ProfileFormProps) {
  const [validationMessage, setValidationMessage] = useState("");
  const [successMessage, setSuccessMessage] = useState("");
  const [primaryLang, setPrimaryLang] = useState<OptionType[]>([]);
  const [secondaryLang, setSecondaryLang] = useState<OptionType[]>([]);
  const [profile, setProfile] = useProfile();
  const [devForm, setDevForm] = useState<DevFormData | null>(null);
  const [empForm, setEmpForm] = useState<EmpFormData | null>(null);
  const [disableSubmit, setDisableSubmit] = useState(false);
  const mdRef = useRef<MDXEditorMethods>(null);
  const [companySelectOptions, setCompanySelectOptions] = useState<
    SelectOptionType[]
  >([]);
  const [selectedCompany, setSelectedCompany] = useState<SelectOptionType>();

  useEffect(() => {
    getLanguages().then((languages) => {
      setPrimaryLang([
        { name: "Select Language", value: 0 },
        ...languages.map((l) => ({ name: l.name, value: l.id })),
      ]);
      setSecondaryLang([
        { name: "Select Language", value: 0 },
        ...languages.map((l) => ({ name: l.name, value: l.id })),
      ]);
    });
  }, []);

  useEffect(() => {
    if (profile) {
      if (userType === UiDevOrEmployer.Developer) {
        const dev = profile as DevProfile;
        setDevForm({
          userName: dev.userName,
          fullName: dev.fullName,
          email: dev.email,
          description: dev.description,
          password: "**********",
          primaryLangId: dev.primaryLangId,
          secondaryLangId: dev.secondaryLangId,
        });
        mdRef.current?.setMarkdown(dev.description);
      } else {
        getCompanies().then((companies) => {
          setCompanySelectOptions(
            companies.map((company) => ({
              value: company.id,
              label: company.name,
            }))
          );

          const emp = profile as EmpProfile;
          const selectedCo = {
            value: emp.companyId,
            label: companies.find((cs) => cs.id == emp.companyId)?.name || "",
          };
          console.log("selectedCo", selectedCo);
          setSelectedCompany(selectedCo);
          setEmpForm({
            userName: emp.userName,
            fullName: emp.fullName,
            email: emp.email,
            password: "**********",
            companyId: emp.companyId,
          });
        });
      }
    }
  }, [profile]);

  const selectCustomStyles: StylesConfig<SelectOptionType, false> = {
    control: (provided, state) => ({
      ...provided,
      backgroundColor: "white",
      borderColor: state.isFocused ? "gray" : "gray",
      minWidth: isModalMode ? "100%" : "250px",
      "div:focus": "none",
    }),
    option: (provided, state) => ({
      ...provided,
      color: state.isSelected ? "red" : "black",
      backgroundColor: state.isSelected ? "lightgray" : "white",
    }),
    // Add more custom styles for other parts
  };

  const createOrEditProfile = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!isValidProfile()) return;

    try {
      setDisableSubmit(true);
      if (userType === UiDevOrEmployer.Developer) {
        if (!devForm) {
          throw new Error("Developer form is null");
        }
        if (editMode === ProfileFormEditMode.Create) {
          if (profile) {
            setValidationMessage(
              "You cannot create a new profile when you already have one"
            );
            return;
          }

          await createDeveloper({
            userName: devForm.userName,
            fullName: devForm.fullName,
            email: devForm.email,
            description: devForm.description,
            password: devForm.password,
            primaryLangId: devForm.primaryLangId,
            secondaryLangId: devForm.secondaryLangId,
          });
        } else {
          if (!profile || !profile.accessToken) {
            setValidationMessage(
              "User must be logged in before making changes to profile"
            );
            return;
          }
          const updateResult = await updateDeveloper({
            id: Number(profile.id),
            fullName: devForm.fullName,
            email: devForm.email,
            description: devForm.description,
            primaryLangId: devForm.primaryLangId,
            secondaryLangId: devForm.secondaryLangId,
            access_token: profile.accessToken,
          });
          if (updateResult) {
            setSuccessMessage("Your profile has been updated.");
            setValidationMessage("");
            const dev = await getDeveloperByEmail(
              devForm.email,
              profile.accessToken
            );
            if (dev) {
              const updatedProfile = convertDev(dev, profile.accessToken);
              setProfile(updatedProfile);
            }
          } else {
            setValidationMessage("Failed to update your profile");
            setSuccessMessage("");
          }
        }
      } else {
        if (!empForm) {
          throw new Error("Employer form is null");
        }
        if (editMode === ProfileFormEditMode.Create) {
          if (profile) {
            setValidationMessage(
              "You cannot create a new profile when you already have one"
            );
            return;
          }

          await createEmployer({
            userName: empForm.userName,
            fullName: empForm.fullName,
            email: empForm.email,
            password: empForm.password,
            companyId: empForm.companyId,
          });
        } else {
          if (!profile || !profile.accessToken) {
            setValidationMessage(
              "User must be logged in before making changes to profile"
            );
            return;
          }
          const updateResult = await updateEmployer({
            id: Number(profile.id),
            fullName: empForm.fullName,
            email: empForm.email,
            companyId: empForm.companyId,
            access_token: profile.accessToken,
          });
          if (updateResult) {
            setSuccessMessage("Your profile has been updated.");
            setValidationMessage("");
            const emp = await getEmployerByEmail(
              empForm.email,
              profile.accessToken
            );
            if (emp) {
              const updatedProfile = convertEmp(emp, profile.accessToken);
              setProfile(updatedProfile);
            }
          } else {
            setValidationMessage("Failed to update your profile");
            setSuccessMessage("");
          }
        }
      }
    } catch (e) {
      setSuccessMessage("");
      if (e instanceof Error) {
        setValidationMessage(e.message);
      }
      setValidationMessage("An error has occurred creating your profile");
    } finally {
      setDisableSubmit(false);
    }
  };

  const isValidProfile = () => {
    setSuccessMessage("");

    if (userType === UiDevOrEmployer.Developer) {
      if (!devForm) throw new Error("Developer form is null");

      if (!devForm.userName) {
        setValidationMessage("Username cannot be empty");
        return false;
      }
      if (devForm.userName.length < 2 || devForm.userName.length > 60) {
        setValidationMessage(
          "Username cannot be shorter than 2 or longer than 60 characters"
        );
        return false;
      }
      if (!devForm.fullName) {
        setValidationMessage("Fullname cannot be empty");
        return false;
      }
      if (devForm.fullName.length < 5 || devForm.userName.length > 100) {
        setValidationMessage(
          "Fullname cannot be shorter than 5 or longer than 100 characters"
        );
        return false;
      }
      if (devForm.description.length < 5 || devForm.description.length > 5000) {
        setValidationMessage(
          "Description cannot be shorter than 5 or longer than 5000 characters"
        );
        return false;
      }
      if (!devForm.email) {
        setValidationMessage("Email cannot be empty");
        return false;
      }
      if (devForm.email.length < 8 || devForm.email.length > 120) {
        setValidationMessage(
          "Email cannot be shorter than 10 or longer than 120 characters"
        );
        return false;
      }

      if (isModalMode) {
        if (!devForm.password) {
          setValidationMessage("Password cannot be empty");
          return false;
        }
        if (devForm.password.length < 8 || devForm.email.length > 50) {
          setValidationMessage(
            "Password cannot be shorter than 8 or longer than 50 characters"
          );
          return false;
        }
      }

      if (!devForm.primaryLangId) {
        setValidationMessage("Primary Language must be selected");
        return false;
      }
    } else {
      if (!empForm) throw new Error("Employer form is null");

      if (empForm.fullName.length < 5 || empForm.userName.length > 100) {
        setValidationMessage(
          "Fullname cannot be shorter than 5 or longer than 100 characters"
        );
        return false;
      }
      if (empForm.companyId.length === 0 || empForm.companyId.length > 120) {
        setValidationMessage(
          "Company name cannot be less than one character or longer than 120 characters"
        );
        return false;
      }
      if (empForm.email.length < 8 || empForm.email.length > 120) {
        setValidationMessage(
          "Email cannot be shorter than 10 or longer than 120 characters"
        );
        return false;
      }
      if (isModalMode) {
        if (!empForm.password) {
          setValidationMessage("Password cannot be empty");
          return false;
        }
        if (empForm.password.length < 8 || empForm.email.length > 50) {
          setValidationMessage(
            "Password cannot be shorter than 8 or longer than 50 characters"
          );
          return false;
        }
      }
    }

    setValidationMessage("");
    return true;
  };

  const updateForm = (e: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    if (userType === UiDevOrEmployer.Developer) {
      if (!devForm) throw new Error("Developer form is null");
      setDevForm({
        ...devForm,
        [name]: value,
      });
    } else {
      if (!empForm) throw new Error("Employer form is null");
      setEmpForm({
        ...empForm,
        [name]: value,
      });
    }
  };

  const onChangeUserName = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    updateForm(e);
  };

  const onChangeFullName = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    updateForm(e);
  };

  const onChangeEmail = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    updateForm(e);
  };

  const onChangePassword = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    updateForm(e);
  };

  const onChangePrimaryLang = (e: ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    if (!devForm) throw new Error("Developer form is null");
    setDevForm({
      ...devForm,
      primaryLangId: Number(e.currentTarget.value),
    });
  };

  const onChangeSecondaryLang = (e: ChangeEvent<HTMLSelectElement>) => {
    e.preventDefault();

    if (!devForm) throw new Error("Developer form is null");
    setDevForm({
      ...devForm,
      secondaryLangId: Number(e.currentTarget.value),
    });
  };

  const onChangeSelectedCompany = (
    newValue: SingleValue<SelectOptionType>,
    _actionMeta: ActionMeta<SelectOptionType>
  ) => {
    if (!empForm) throw new Error("Employer form is null");
    console.log("newValue", newValue);
    console.log("actionMeta", _actionMeta);
    const newSelected = {
      value: newValue?.value || "",
      label: newValue?.label || "",
    };
    setSelectedCompany(newSelected);
    setEmpForm({
      ...empForm,
      companyId: newSelected.value,
    });
  };

  const getMarkdownText = (markdown: string) => {
    if (!devForm) throw new Error("Developer form is null");
    setDevForm({
      ...devForm,
      description: markdown,
    });
  };

  return (
    <div style={{ width: "100%" }}>
      <form className="login-form" onSubmit={createOrEditProfile}>
        {isModalMode ? (
          <div className="login-item">
            <span className="title-font">Welcome to SyntaxMakers</span>
            <span
              className="sub-title-font"
              style={{ color: "var(--primary-font-cl)" }}
            >
              Please register
            </span>
          </div>
        ) : (
          <header className="header-container job-full-view-header">
            <strong>
              {`@${
                userType === UiDevOrEmployer.Developer
                  ? devForm
                    ? devForm.userName
                    : ""
                  : empForm
                  ? empForm.userName
                  : ""
              }`}{" "}
              Profile
            </strong>
          </header>
        )}
        <div style={{ padding: isModalMode ? "" : "2em", width: "100%" }}>
          <section className="form-section">
            <span>Username</span>
            <input
              type="text"
              name="userName"
              className="input normal-font input-spacing"
              style={
                isModalMode
                  ? { width: "45%" }
                  : {
                      width: "75%",
                      backgroundColor: "var(--border-cl)",
                      opacity: 0.75,
                    }
              }
              value={
                userType === UiDevOrEmployer.Developer
                  ? devForm
                    ? devForm.userName
                    : ""
                  : empForm
                  ? empForm.userName
                  : ""
              }
              onChange={onChangeUserName}
              disabled={isModalMode ? false : true}
            />
          </section>
          <section className="form-section">
            <span>Fullname</span>
            <input
              type="text"
              name="fullName"
              className="input normal-font input-spacing"
              style={isModalMode ? { width: "45%" } : { width: "75%" }}
              value={
                userType === UiDevOrEmployer.Developer
                  ? devForm
                    ? devForm.fullName
                    : ""
                  : empForm
                  ? empForm.fullName
                  : ""
              }
              onChange={onChangeFullName}
            />
          </section>
          {userType === UiDevOrEmployer.Developer ? (
            <section className="form-section">
              <span>Description</span>
              <div
                style={
                  isModalMode
                    ? { marginTop: "1em", marginBottom: "3em", width: "45%" }
                    : { marginTop: "1em", marginBottom: "3em", width: "75%" }
                }
              >
                <MarkdownEditor
                  mdRef={mdRef}
                  readOnly={false}
                  markdown={devForm ? devForm.description : ""}
                  getChangedText={getMarkdownText}
                />
              </div>
            </section>
          ) : (
            <section className="form-section" style={{ marginBottom: "1em" }}>
              <span>Company</span>
              <CreateableSelect
                isClearable
                styles={selectCustomStyles}
                options={companySelectOptions}
                value={selectedCompany}
                defaultValue={selectedCompany}
                onChange={onChangeSelectedCompany}
              />
            </section>
          )}
          <section className="form-section">
            <span>Email</span>
            <input
              type="text"
              name="email"
              className="input normal-font input-spacing"
              style={isModalMode ? { width: "45%" } : { width: "75%" }}
              value={
                userType === UiDevOrEmployer.Developer
                  ? devForm
                    ? devForm.email
                    : ""
                  : empForm
                  ? empForm.email
                  : ""
              }
              onChange={onChangeEmail}
            />
          </section>
          {isModalMode ? (
            <section className="form-section">
              <span>Password</span>
              <input
                type="password"
                name="password"
                className="input normal-font input-spacing"
                style={isModalMode ? { width: "45%" } : { width: "75%" }}
                value={
                  userType === UiDevOrEmployer.Developer
                    ? devForm
                      ? devForm.password
                      : ""
                    : empForm
                    ? empForm.password
                    : ""
                }
                onChange={onChangePassword}
              />
            </section>
          ) : null}

          {userType === UiDevOrEmployer.Developer ? (
            <>
              <section className="form-section" style={{ marginBottom: "1em" }}>
                <DropDown
                  keyName="devprimarylang"
                  name="devprimarylang"
                  value={devForm ? devForm.primaryLangId : ""}
                  label="Primary Language"
                  optionItems={primaryLang}
                  onChange={onChangePrimaryLang}
                  selectStyle={
                    isModalMode
                      ? { marginLeft: ".5em", width: "45%" }
                      : { width: "75%" }
                  }
                  isHorizontal={true}
                />
              </section>
              <section className="form-section">
                <DropDown
                  keyName="devsecondarylang"
                  name="devsecondarylang"
                  value={devForm ? devForm.secondaryLangId : ""}
                  label="Secondary Language"
                  optionItems={secondaryLang}
                  onChange={onChangeSecondaryLang}
                  selectStyle={
                    isModalMode
                      ? { marginLeft: ".5em", width: "45%" }
                      : { width: "75%" }
                  }
                  isHorizontal={true}
                />
              </section>
            </>
          ) : null}

          <section
            className="form-section"
            style={{ marginTop: "1.5em", justifyContent: "flex-end" }}
          >
            <PrimaryButton type="submit" disabled={disableSubmit}>
              {editMode === ProfileFormEditMode.Edit ? "edit" : "create"}
            </PrimaryButton>
          </section>
          <ValidationMsgView
            validationMessage={validationMessage}
            successMessage={successMessage}
          />
        </div>
      </form>
      {!isModalMode ? (
        <div style={{ marginTop: "2em" }}>
          <ChangePassword userType={userType} />
        </div>
      ) : null}
    </div>
  );
}
