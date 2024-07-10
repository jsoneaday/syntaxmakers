import Modal from "../Modal";
import { useEffect, useState } from "react";
import { CONFIRM_EMAIL_URL } from "../../../domain/repository/Api";
import { useNavigate, useSearchParams } from "react-router-dom";
import { ValidationMsgView } from "../controls/ValidationMsgView";
import { RoutePaths } from "../../../App";
import { PrimaryButton } from "../controls/Buttons";

interface ConfirmEmailModalProps {
  isOpen: boolean;
  toggleOpen: () => void;
}

export function ConfirmEmailModal({
  isOpen,
  toggleOpen,
}: ConfirmEmailModalProps) {
  const [successMsg, setSuccessMsg] = useState("");
  const [validationMsg, setValidationMsg] = useState("");
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  useEffect(() => {
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
  }, []);

  const onClick = () => {
    const isDev = searchParams.get("is_dev");
    if (isDev === "true") {
      navigate(RoutePaths.DevJobSearch);
    } else {
      navigate(RoutePaths.EmpJobPosts);
    }
  };

  return (
    <Modal isOpen={isOpen} toggleOpen={toggleOpen}>
      <div className="popup-header">
        <header style={{ alignSelf: "flex-start" }}>
          <strong>Email Confirmation</strong>
        </header>
      </div>
      <div className="popup-body">
        <ValidationMsgView
          successMessage={successMsg}
          validationMessage={validationMsg}
        />
        {successMsg ? (
          <PrimaryButton onClick={onClick}>Login</PrimaryButton>
        ) : null}
      </div>
    </Modal>
  );
}
