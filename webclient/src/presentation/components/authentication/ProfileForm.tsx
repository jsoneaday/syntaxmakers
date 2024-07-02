import { ChangeEvent, FormEvent, useEffect, useState } from "react";
import { DevOrEmployer } from "../../models/DevOrEmployer";
import DropDown, { OptionType } from "../controls/DropDown";
import { getLanguages } from "../../../domain/repository/LanguageRepo";
import { createDeveloper } from "../../../domain/repository/DeveloperRepo";

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
  secondaryLangId: number | undefined;
}

interface ProfileFormProps {
  isModalMode: boolean;
  editMode: ProfileFormEditMode;
  toggleOpen: () => void;
  userType: DevOrEmployer;
}

export function ProfileForm({
  isModalMode,
  toggleOpen,
  editMode,
  userType,
}: ProfileFormProps) {
  const [validationMessage, setValidationMessage] = useState("");
  const [successMessage, setSuccessMessage] = useState("");
  const [primaryLang, setPrimaryLang] = useState<OptionType[]>([]);
  const [secondaryLang, setSecondaryLang] = useState<OptionType[]>([]);
  const [profile, setProfile] = useState<ProfileFormData>({
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

  const createOrEditProfile = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!isValidateProfile()) return;

    if (editMode === ProfileFormEditMode.Create) {
      console.log("profile form", profile);

      try {
        await createDeveloper({
          userName: profile.userName,
          fullName: profile.fullName,
          email: profile.email,
          password: profile.password,
          primaryLangId: profile.primaryLangId,
          secondaryLangId: profile.secondaryLangId,
        });
        setSuccessMessage(
          "Your profile has been created please check your email for confirmation."
        );
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
    if (!profile.userName) {
      setValidationMessage("Username cannot be empty");
      return false;
    }
    if (profile.userName.length < 2 || profile.userName.length > 60) {
      setValidationMessage(
        "Username cannot be shorter than 2 or longer than 60 characters"
      );
      return false;
    }
    if (!profile.fullName) {
      setValidationMessage("Fullname cannot be empty");
      return false;
    }
    if (profile.fullName.length < 5 || profile.userName.length > 100) {
      setValidationMessage(
        "Fullname cannot be shorter than 5 or longer than 100 characters"
      );
      return false;
    }
    if (!profile.email) {
      setValidationMessage("Email cannot be empty");
      return false;
    }
    if (profile.email.length < 8 || profile.email.length > 120) {
      setValidationMessage(
        "Email cannot be shorter than 10 or longer than 120 characters"
      );
      return false;
    }
    if (!profile.password) {
      setValidationMessage("Password cannot be empty");
      return false;
    }
    if (profile.password.length < 8 || profile.email.length > 40) {
      setValidationMessage(
        "Password cannot be shorter than 8 or longer than 40 characters"
      );
      return false;
    }
    console.log("primaryLangId", profile.primaryLangId);
    if (!profile.primaryLangId) {
      console.log("failed");
      setValidationMessage("Primary Language must be selected");
      return false;
    }

    setValidationMessage("");
    return true;
  };

  const updateForm = (e: ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;

    setProfile({
      ...profile,
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
    setProfile({
      ...profile,
      primaryLangId: Number(e.currentTarget.value),
    });
  };

  const onChangeSecondaryLang = (e: ChangeEvent<HTMLSelectElement>) => {
    console.log("secondary id", e.currentTarget.value);
    e.preventDefault();
    setProfile({
      ...profile,
      secondaryLangId: Number(e.currentTarget.value),
    });
  };

  return (
    <form className="login-form" onSubmit={createOrEditProfile}>
      <section>
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
            <strong>{profile.userName} Profile</strong>
          </header>
        )}
      </section>
      <section className="form-section">
        <span>Username</span>
        <input
          type="text"
          name="userName"
          className="input normal-font input-spacing"
          style={isModalMode ? { width: "45%" } : {}}
          value={profile.userName}
          onChange={onChangeUserName}
        />
      </section>
      <section className="form-section">
        <span>Fullname</span>
        <input
          type="text"
          name="fullName"
          className="input normal-font input-spacing"
          style={isModalMode ? { width: "45%" } : {}}
          value={profile.fullName}
          onChange={onChangeFullName}
        />
      </section>
      <section className="form-section">
        <span>Email</span>
        <input
          type="text"
          name="email"
          className="input normal-font input-spacing"
          style={isModalMode ? { width: "45%" } : {}}
          value={profile.email}
          onChange={onChangeEmail}
        />
      </section>
      <section className="form-section">
        <span>Password</span>
        <input
          type="password"
          name="password"
          className="input normal-font input-spacing"
          style={isModalMode ? { width: "45%" } : {}}
          value={profile.password}
          onChange={onChangePassword}
        />
      </section>
      <section className="form-section" style={{ marginBottom: "1em" }}>
        <DropDown
          keyName="devprimarylang"
          name="devprimarylang"
          value={profile.primaryLangId}
          label="Primary Language"
          optionItems={primaryLang}
          onChange={onChangePrimaryLang}
          selectStyle={isModalMode ? { marginLeft: ".5em", width: "45%" } : {}}
          isHorizontal={true}
        />
      </section>
      <section className="form-section">
        <DropDown
          keyName="devsecondarylang"
          name="devsecondarylang"
          value={profile.secondaryLangId}
          label="Secondary Language"
          optionItems={secondaryLang}
          onChange={onChangeSecondaryLang}
          selectStyle={isModalMode ? { marginLeft: ".5em", width: "45%" } : {}}
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
    </form>
  );
}
