import { JobApplicantModel } from "../../../models/JobApplicantModel";
import { PrimaryButton } from "../../controls/Buttons";

interface ViewProfileProps {
  profile: JobApplicantModel;
}

export function PreviewApplicant({ profile }: ViewProfileProps) {
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
            {profile.appliedAt}
          </small>
        </section>
        <section className="form-section">
          <span>Full Name</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {profile.fullName}
          </span>
        </section>
        <section className="form-section">
          <span>Primary Language</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {profile.primaryLangName}
          </span>
        </section>
        <section className="form-section">
          <span>Secondary Language</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {profile.secondaryLangName}
          </span>
        </section>
        <section className="form-section">
          <span>Description</span>
          <span
            style={{
              width: "75%",
            }}
          >
            {profile.description}
          </span>
        </section>
        <section
          className="form-section"
          style={{ marginTop: "1em", marginBottom: "1em" }}
        >
          <PrimaryButton>message</PrimaryButton>
        </section>
      </div>
    </div>
  );
}
