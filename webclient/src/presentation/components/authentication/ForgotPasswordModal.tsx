import Modal from "../Modal";
import { ChangeEvent, useEffect, useState } from "react";
import { CONFIRM_EMAIL_URL } from "../../../domain/repository/Api";
import { useNavigate, useSearchParams } from "react-router-dom";
import { ValidationMsgView } from "../controls/ValidationMsgView";
import { RoutePaths } from "../../../App";
import { PrimaryButton } from "../controls/Buttons";

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
  const navigate = useNavigate();
  const [password, setPassword] = useState("");

  const onClick = () => {
    const is_dev = searchParams.get("is_dev");
    const profile_id = searchParams.get("profile_id");
    const new_email = searchParams.get("new_email");
    const unique_key = searchParams.get("unique_key");
    console.log("variables:", is_dev, profile_id, new_email, unique_key);
    fetch(
      `${CONFIRM_EMAIL_URL}?is_dev=${is_dev}&profile_id=${profile_id}&new_email=${new_email}&unique_key=${unique_key}`
    )
      .then((response) => {
        console.log("response", response);
        if (response.ok) {
          setValidationMsg("");
          setSuccessMsg("Your email is confirmed. You can login now.");

          return;
        } else {
          response.text().then((text) => {
            setValidationMsg(text);
            setSuccessMsg("");
          });
        }
      })
      .catch((e) => {
        if (e instanceof Error) {
          setValidationMsg(e.message);
          setSuccessMsg("");
        }
      });
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
      <div className="popup-body">
        <input
          type="password"
          className="input normal-font input-spacing"
          value={password}
          onChange={onChangePassword}
        />
        <ValidationMsgView
          successMessage={successMsg}
          validationMessage={validationMsg}
        />
        {successMsg ? (
          <PrimaryButton onClick={onClick}>change</PrimaryButton>
        ) : null}
      </div>
    </Modal>
  );
}
