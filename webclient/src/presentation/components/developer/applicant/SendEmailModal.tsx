import Modal from "../../Modal";
import { ChangeEvent, useState } from "react";
import { ValidationMsgView } from "../../controls/ValidationMsgView";
import { PrimaryButton } from "../../controls/Buttons";
import { useProfile } from "../../../common/redux/profile/ProfileHooks";
import { MouseEvent } from "react";
import { sendEmail } from "../../../../domain/repository/UserRepo";

interface SendEmailModalProps {
  isOpen: boolean;
  toggleOpen: () => void;
  receiverDevId: string;
}

export function SendEmailModal({
  isOpen,
  toggleOpen,
  receiverDevId,
}: SendEmailModalProps) {
  const [profile] = useProfile();
  const [successMsg, setSuccessMsg] = useState("");
  const [validationMsg, setValidationMsg] = useState("");
  const [subject, setSubject] = useState("");
  const [body, setBody] = useState("");
  const [disabled, setDisabled] = useState(false);

  const onClickSendEmail = async (e: MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();

    if (!profile) {
      setValidationMsg("You must be logged in to send emails");
      setSuccessMsg("");
      return;
    }

    try {
      setDisabled(true);
      const sendResult = await sendEmail(
        profile.id,
        receiverDevId,
        subject,
        body,
        profile.accessToken || ""
      );
      if (sendResult) {
        setValidationMsg("");
        setSuccessMsg("Your email was sent");
      } else {
        setValidationMsg("Email failed to send");
        setSuccessMsg("");
        setDisabled(false);
      }
    } catch (e) {
      setValidationMsg("Email failed to send");
      setSuccessMsg("");
      setDisabled(false);
    }
  };

  const onChangeSubject = (e: ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();

    setSubject(e.target.value);
  };

  const onChangeBody = (e: ChangeEvent<HTMLTextAreaElement>) => {
    e.preventDefault();

    setBody(e.target.value);
  };

  return (
    <Modal isOpen={isOpen} toggleOpen={toggleOpen}>
      <div className="login-form" style={{ width: "100%" }}>
        <section className="form-section" style={{ marginBottom: "1em" }}>
          <header style={{ alignSelf: "flex-start" }}>
            <strong>Send Applicant Email</strong>
          </header>
        </section>
        <section className="form-section" style={{ marginBottom: "1em" }}>
          <input
            type="text"
            className="input normal-font input-spacing"
            value={subject}
            onChange={onChangeSubject}
          />
        </section>
        <section className="form-section" style={{ marginBottom: "1em" }}>
          <textarea
            value={body}
            className="input normal-font input-spacing"
            style={{ height: "10em", fontFamily: "Arial" }}
            onChange={onChangeBody}
          />
        </section>
        <section
          className="form-section"
          style={{ justifyContent: "flex-end" }}
        >
          <PrimaryButton onClick={onClickSendEmail} disabled={disabled}>
            Login
          </PrimaryButton>
        </section>
        <ValidationMsgView
          successMessage={successMsg}
          validationMessage={validationMsg}
        />
      </div>
    </Modal>
  );
}
