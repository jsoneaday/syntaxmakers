import { JobApplicant } from "../../../models/JobApplicant";

interface ViewProfileProps {
  profile: JobApplicant;
}

export function PreviewProfile({ profile }: ViewProfileProps) {
  return (
    <div className="login-form">
      <section className="form-section">
        <span>Full Name</span>
        <input
          type="text"
          className="input normal-font input-spacing"
          style={{
            width: "75%",
            backgroundColor: "var(--border-cl)",
            opacity: 0.75,
          }}
          readOnly
          value={profile.fullName}
        />
      </section>
      <section className="form-section">
        <span>Primary Language</span>
        <input
          type="text"
          className="input normal-font input-spacing"
          style={{
            width: "75%",
            backgroundColor: "var(--border-cl)",
            opacity: 0.75,
          }}
          readOnly
          value={profile.primaryLangName}
        />
      </section>
      <section className="form-section">
        <span>Description</span>
        <input
          type="text"
          className="input normal-font input-spacing"
          style={{
            width: "75%",
            backgroundColor: "var(--border-cl)",
            opacity: 0.75,
          }}
          readOnly
          value={profile.description}
        />
      </section>
    </div>
  );
}
