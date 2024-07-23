import Modal from "../Modal";
import { ChangeEvent, MouseEvent, useState } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { ValidationMsgView } from "../controls/ValidationMsgView";
import { PrimaryButton } from "../controls/Buttons";
import { resetPassword } from "../../../domain/repository/UserRepo";
import { DevOrEmployer } from "../../../domain/repository/AuthRepo";
import { RoutePaths } from "../../../App";

interface ForgotPasswordModalProps {
  isOpen: boolean;
  toggleOpen: () => void;
}

export function ForgotPasswordModal({
  isOpen,
  toggleOpen,
}: ForgotPasswordModalProps) {
  const [successMsg, setSuccessMsg] = useState("");
  const [validationMsg, setValidationMsg] = useState("");
  const [searchParams] = useSearchParams();
  const [password, setPassword] = useState("");
  const [login, setLogin] = useState(false);
  const [disableReset, setDisableReset] = useState(false);
  const navigate = useNavigate();

  const onClickReset = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    if (!password || password.length < 8 || password.length > 50) {
      setValidationMsg(
        "Passwords must be at least 8 characters and less than 50."
      );
      setSuccessMsg("");
      return;
    }

    setDisableReset(true);
    const is_dev = searchParams.get("is_dev");
    const profile_id = searchParams.get("profile_id");
    const unique_key = searchParams.get("unique_key");
    console.log("variables:", is_dev, profile_id, password, unique_key);

    const result = await resetPassword(
      Number(profile_id || 0),
      password,
      is_dev === "true" ? DevOrEmployer.Developer : DevOrEmployer.Employer,
      unique_key || ""
    );

    if (result.result) {
      setSuccessMsg("Password reset please login.");
      setValidationMsg("");
      setLogin(true);
      setDisableReset(false);
      return;
    }
    setSuccessMsg("");
    setValidationMsg(
      result.message
        ? result.message
        : "Password reset failed. Please try again."
    );
    setLogin(false);
    setDisableReset(false);
  };

  const onClickLogin = (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const isDev = searchParams.get("is_dev");
    if (isDev === "true") {
      navigate(RoutePaths.DevJobSearch);
    } else {
      navigate(RoutePaths.EmpJobPosts);
    }
  };

  const onChangePassword = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    setPassword(e.target.value);
  };

  return (
    <Modal isOpen={isOpen} toggleOpen={toggleOpen}>
      <div className="popup-header">
        <header style={{ alignSelf: "flex-start" }}>
          <strong>Reset Password</strong>
        </header>
      </div>
      <div
        className="popup-body"
        style={{ alignItems: "flex-end", marginBottom: "3em" }}
      >
        <section className="login-item">
          <span className="login-label">New Password</span>
          <input
            type="password"
            className="input normal-font input-spacing"
            value={password}
            onChange={onChangePassword}
          />
        </section>
        <div
          style={{
            width: "100%",
            textAlign: "right",
            marginBottom: "1em",
          }}
        >
          {login ? (
            <PrimaryButton onClick={onClickLogin}>Login</PrimaryButton>
          ) : (
            <PrimaryButton disabled={disableReset} onClick={onClickReset}>
              Change
            </PrimaryButton>
          )}
        </div>
        <ValidationMsgView
          successMessage={successMsg}
          validationMessage={validationMsg}
        />
      </div>
    </Modal>
  );
}
