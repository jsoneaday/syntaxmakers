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
import DevProfile, { convert } from "../../models/DevProfile";
import { ValidationMsgView } from "../controls/ValidationMsgView";
import { ChangePassword } from "./ChangePassword";
import { PrimaryButton } from "../controls/Buttons";
import { MarkdownEditor } from "../textEditor/MarkdownEditor";
import { MDXEditorMethods } from "@mdxeditor/editor";

export enum ProfileFormEditMode {
  Create,
  Edit,
}

interface ProfileFormData {
  userName: string;
  fullName: string;
  email: string;
  description: string;
  password: string;
  primaryLangId: number;
  secondaryLangId?: number | null;
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
  const [profileForm, setProfileForm] = useState<ProfileFormData>({
    userName: "",
    fullName: "",
    email: "",
    description: "",
    password: "",
    primaryLangId: 0,
    secondaryLangId: undefined,
  });
  const [disableSubmit, setDisableSubmit] = useState(false);
  const mdRef = useRef<MDXEditorMethods>(null);

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
        setProfileForm({
          userName: dev.userName,
          fullName: dev.fullName,
          email: dev.email,
          description: dev.description,
          password: "**********",
          primaryLangId: dev.primaryLangId,
          secondaryLangId: dev.secondaryLangId,
        });
        mdRef.current?.setMarkdown(dev.description);
      }
    }
  }, [profile]);

  const createOrEditProfile = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!isValidateProfile()) return;

    try {
      setDisableSubmit(true);
      if (userType === UiDevOrEmployer.Developer) {
        if (editMode === ProfileFormEditMode.Create) {
          if (profile) {
            setValidationMessage(
              "You cannot create a new profile when you already have one"
            );
            return;
          }
          await createDeveloper({
            userName: profileForm.userName,
            fullName: profileForm.fullName,
            email: profileForm.email,
            description: profileForm.description,
            password: profileForm.password,
            primaryLangId: profileForm.primaryLangId,
            secondaryLangId: profileForm.secondaryLangId,
          });
        } else {
          if (!profile || !profile.accessToken) {
            setValidationMessage(
              "User must be logged in before making changes to profile"
            );
            return;
          }
          const result = await updateDeveloper({
            id: Number(profile.id),
            fullName: profileForm.fullName,
            email: profileForm.email,
            description: profileForm.description,
            primaryLangId: profileForm.primaryLangId,
            secondaryLangId: profileForm.secondaryLangId,
            access_token: profile.accessToken,
          });
          if (result) {
            setSuccessMessage("Your profile has been updated.");
            setValidationMessage("");
            const dev = await getDeveloperByEmail(
              profileForm.email,
              profile.accessToken
            );
            if (dev) {
              const updatedProfile = convert(dev, profile.accessToken);
              setProfile(updatedProfile);
            }
          } else {
            setValidationMessage("Failed to update your profile");
            setSuccessMessage("");
          }
        }
      }

      setValidationMessage("");
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

  const isValidateProfile = () => {
    setSuccessMessage("");
    if (!profileForm.userName) {
      setValidationMessage("Username cannot be empty");
      return false;
    }
    if (profileForm.userName.length < 2 || profileForm.userName.length > 60) {
      setValidationMessage(
        "Username cannot be shorter than 2 or longer than 60 characters"
      );
      return false;
    }
    if (!profileForm.fullName) {
      setValidationMessage("Fullname cannot be empty");
      return false;
    }
    if (profileForm.fullName.length < 5 || profileForm.userName.length > 100) {
      setValidationMessage(
        "Fullname cannot be shorter than 5 or longer than 100 characters"
      );
      return false;
    }
    if (
      profileForm.description.length < 5 ||
      profileForm.description.length > 5000
    ) {
      setValidationMessage(
        "Description cannot be shorter than 5 or longer than 5000 characters"
      );
      return false;
    }
    if (!profileForm.email) {
      setValidationMessage("Email cannot be empty");
      return false;
    }
    if (profileForm.email.length < 8 || profileForm.email.length > 120) {
      setValidationMessage(
        "Email cannot be shorter than 10 or longer than 120 characters"
      );
      return false;
    }

    if (isModalMode) {
      if (!profileForm.password) {
        setValidationMessage("Password cannot be empty");
        return false;
      }
      if (profileForm.password.length < 8 || profileForm.email.length > 50) {
        setValidationMessage(
          "Password cannot be shorter than 8 or longer than 50 characters"
        );
        return false;
      }
    }

    if (!profileForm.primaryLangId) {
      setValidationMessage("Primary Language must be selected");
      return false;
    }

    setValidationMessage("");
    return true;
  };

  const updateForm = (e: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    setProfileForm({
      ...profileForm,
      [name]: value,
    });
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
    console.log("primary id", e.currentTarget.value);
    e.preventDefault();
    setProfileForm({
      ...profileForm,
      primaryLangId: Number(e.currentTarget.value),
    });
  };

  const onChangeSecondaryLang = (e: ChangeEvent<HTMLSelectElement>) => {
    console.log("secondary id", e.currentTarget.value);
    e.preventDefault();
    setProfileForm({
      ...profileForm,
      secondaryLangId: Number(e.currentTarget.value),
    });
  };

  const getMarkdownText = (markdown: string) => {
    setProfileForm({
      ...profileForm,
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
            <strong>{`@${profileForm.userName}`} Profile</strong>
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
              value={profileForm.userName}
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
              value={profileForm.fullName}
              onChange={onChangeFullName}
            />
          </section>
          {userType === UiDevOrEmployer.Developer ? (
            <section className="form-section">
              <span>Description</span>
              <div
                style={{ marginTop: "1em", marginBottom: "3em", width: "75%" }}
              >
                <MarkdownEditor
                  mdRef={mdRef}
                  readOnly={false}
                  markdown={profileForm.description}
                  getChangedText={getMarkdownText}
                />
              </div>
            </section>
          ) : null}
          <section className="form-section">
            <span>Email</span>
            <input
              type="text"
              name="email"
              className="input normal-font input-spacing"
              style={isModalMode ? { width: "45%" } : { width: "75%" }}
              value={profileForm.email}
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
                value={profileForm.password}
                onChange={onChangePassword}
              />
            </section>
          ) : null}
          <section className="form-section" style={{ marginBottom: "1em" }}>
            <DropDown
              keyName="devprimarylang"
              name="devprimarylang"
              value={profileForm.primaryLangId}
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
              value={profileForm.secondaryLangId}
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
