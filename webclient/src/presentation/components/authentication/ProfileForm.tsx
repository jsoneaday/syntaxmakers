import { ChangeEvent, FormEvent, useEffect, useState } from "react";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import DropDown, { OptionType } from "../controls/DropDown";
import { getLanguages } from "../../../domain/repository/LanguageRepo";
import { createDeveloper } from "../../../domain/repository/DeveloperRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import DevProfile from "../../models/DevProfile";

export enum ProfileFormEditMode {
  Create,
  Edit,
}

interface ProfileFormData {
  userName: string;
  fullName: string;
  email: string;
  password: string;
  primaryLangId: number;
  secondaryLangId?: number | null;
}

interface ProfileFormProps {
  isModalMode: boolean;
  editMode: ProfileFormEditMode;
  userType: DevOrEmployer;
}

export function ProfileForm({
  isModalMode,
  editMode,
  userType,
}: ProfileFormProps) {
  const [validationMessage, setValidationMessage] = useState("");
  const [successMessage, setSuccessMessage] = useState("");
  const [primaryLang, setPrimaryLang] = useState<OptionType[]>([]);
  const [secondaryLang, setSecondaryLang] = useState<OptionType[]>([]);
  const [profile, _setProfile] = useProfile();
  const [profileForm, setProfileForm] = useState<ProfileFormData>({
    userName: "",
    fullName: "",
    email: "",
    password: "",
    primaryLangId: 0,
    secondaryLangId: undefined,
  });

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
      if (userType === DevOrEmployer.Developer) {
        const dev = profile as DevProfile;
        setProfileForm({
          userName: dev.userName,
          fullName: dev.fullName,
          email: dev.email,
          password: "**********",
          primaryLangId: dev.primaryLangId,
          secondaryLangId: dev.secondaryLangId,
        });
      }
    }
  }, [profile]);

  const createOrEditProfile = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!isValidateProfile()) return;

    if (editMode === ProfileFormEditMode.Create) {
      console.log("profile form", profileForm);

      try {
        if (userType === DevOrEmployer.Developer) {
          await createDeveloper({
            userName: profileForm.userName,
            fullName: profileForm.fullName,
            email: profileForm.email,
            password: profileForm.password,
            primaryLangId: profileForm.primaryLangId,
            secondaryLangId: profileForm.secondaryLangId,
          });
          setSuccessMessage(
            "Your profile has been created please check your email for confirmation."
          );
        }

        setValidationMessage("");
      } catch (e) {
        setSuccessMessage("");
        if (e instanceof Error) {
          setValidationMessage(e.message);
        }
        setValidationMessage("An error has occurred creating your profile");
      }
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
    if (!profileForm.password) {
      setValidationMessage("Password cannot be empty");
      return false;
    }
    if (profileForm.password.length < 8 || profileForm.email.length > 40) {
      setValidationMessage(
        "Password cannot be shorter than 8 or longer than 40 characters"
      );
      return false;
    }
    console.log("primaryLangId", profileForm.primaryLangId);
    if (!profileForm.primaryLangId) {
      console.log("failed");
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

  return (
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
          <strong>{profileForm.userName} Profile</strong>
        </header>
      )}
      <div style={{ padding: "2em", width: "100%" }}>
        <section className="form-section">
          <span>Username</span>
          <input
            type="text"
            name="userName"
            className="input normal-font input-spacing"
            style={isModalMode ? { width: "45%" } : { width: "80%" }}
            value={profileForm.userName}
            onChange={onChangeUserName}
          />
        </section>
        <section className="form-section">
          <span>Fullname</span>
          <input
            type="text"
            name="fullName"
            className="input normal-font input-spacing"
            style={isModalMode ? { width: "45%" } : { width: "80%" }}
            value={profileForm.fullName}
            onChange={onChangeFullName}
          />
        </section>
        <section className="form-section">
          <span>Email</span>
          <input
            type="text"
            name="email"
            className="input normal-font input-spacing"
            style={isModalMode ? { width: "45%" } : { width: "80%" }}
            value={profileForm.email}
            onChange={onChangeEmail}
          />
        </section>
        <section className="form-section">
          <span>Password</span>
          <input
            type="password"
            name="password"
            className="input normal-font input-spacing"
            style={isModalMode ? { width: "45%" } : { width: "80%" }}
            value={profileForm.password}
            onChange={onChangePassword}
          />
        </section>
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
                : { width: "80%" }
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
                : { width: "80%" }
            }
            isHorizontal={true}
          />
        </section>
        <section
          className="form-section"
          style={{ marginTop: "1.5em", justifyContent: "flex-end" }}
        >
          <button type="submit" className="primary-btn">
            {editMode === ProfileFormEditMode.Edit ? "edit" : "create"}
          </button>
        </section>
        <section
          className="form-section"
          style={{
            marginTop: "1.5em",
            color: validationMessage ? "var(--error-cl)" : "",
          }}
        >
          <span>{validationMessage ? validationMessage : successMessage}</span>
        </section>
      </div>
    </form>
  );
}
