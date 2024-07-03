import { ChangeEvent, FormEvent, useState } from "react";
import { ValidationMsgView } from "../controls/ValidationMsgView";
import { changePassword } from "../../../domain/repository/DeveloperRepo";
import { useProfile } from "../../common/redux/profile/ProfileHooks";
import { PrimaryButton } from "../controls/Buttons";

export function ChangePassword() {
  const [profile] = useProfile();
  const [oldPassword, setOldPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [validationMessage, setValidationMessage] = useState("");
  const [successMessage, setSuccessMessage] = useState("");
  const [disableSubmit, setDisableSubmit] = useState(false);

  const onChangeOldPassword = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setOldPassword(e.target.value);
  };

  const onChangeNewPassword = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setNewPassword(e.target.value);
  };

  const onSubmitChangePassword = async (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();

    if (!profile || !profile.accessToken) {
      setValidationMessage("You must be logged in to change your password");
      setSuccessMessage("");
      return;
    }
    if (newPassword.length < 8 || newPassword.length > 50) {
      setValidationMessage(
        "New passwords must be at least 8 characters and less than 50 characters"
      );
      setSuccessMessage("");
      return;
    }

    try {
      setDisableSubmit(true);
      const result = await changePassword(
        Number(profile.id),
        oldPassword,
        newPassword,
        profile.accessToken
      );

      if (result) {
        setSuccessMessage("Password changed successfully");
        setValidationMessage("");
      } else {
        setSuccessMessage("");
        setValidationMessage("Password change failed");
      }
    } catch (e) {
      setSuccessMessage("");
      setValidationMessage("Password change failed");
    } finally {
      setDisableSubmit(false);
    }
  };

  return (
    <form className="login-form" onSubmit={onSubmitChangePassword}>
      <div style={{ padding: "2em", width: "100%" }}>
        <section className="form-section">
          <span>Current Password</span>
          <input
            type="password"
            name="password"
            className="input normal-font input-spacing"
            style={{ width: "75%" }}
            value={oldPassword}
            onChange={onChangeOldPassword}
          />
        </section>
        <section className="form-section">
          <span>New Password</span>
          <input
            type="password"
            name="password"
            className="input normal-font input-spacing"
            style={{ width: "75%" }}
            value={newPassword}
            onChange={onChangeNewPassword}
          />
        </section>
        <section
          className="form-section"
          style={{ marginTop: "1.5em", justifyContent: "flex-end" }}
        >
          <PrimaryButton type="submit" disabled={disableSubmit}>
            change
          </PrimaryButton>
        </section>
        <ValidationMsgView
          validationMessage={validationMessage}
          successMessage={successMessage}
        />
      </div>
    </form>
  );
}
