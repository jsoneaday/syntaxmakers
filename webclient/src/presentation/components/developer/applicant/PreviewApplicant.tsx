import { JobApplicantModel } from "../../../models/JobApplicantModel";
import { PrimaryButton } from "../../controls/Buttons";

interface PreviewApplicantProps {
  applicant: JobApplicantModel;
  selectDevId: (devId: string) => void;
}

export function PreviewApplicant({
  applicant,
  selectDevId,
}: PreviewApplicantProps) {
  const onClickToggleEmail = () => {
    selectDevId(applicant.devId);
  };

  return (
    <div style={{ width: "100%" }}>
      <div className="login-form">
        <section className="form-section">
          <span>Applied At</span>
          <small
            style={{
              width: "75%",
            }}
          >
            {applicant.appliedAt}
          </small>
        </section>
        <section className="form-section">
          <span>Full Name</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {applicant.fullName}
          </span>
        </section>
        <section className="form-section">
          <span>Primary Language</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {applicant.primaryLangName}
          </span>
        </section>
        <section className="form-section">
          <span>Secondary Language</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {applicant.secondaryLangName}
          </span>
        </section>
        <section className="form-section">
          <span>Description</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {applicant.description}
          </span>
        </section>
        <section
          className="form-section"
          style={{ marginTop: "1em", marginBottom: "1em" }}
        >
          <PrimaryButton onClick={onClickToggleEmail}>email</PrimaryButton>
        </section>
      </div>
    </div>
  );
}
