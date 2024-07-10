import { useState } from "react";
import { ViewableDevProfile } from "../../../models/ViewProfileModel";
import { MarkdownEditor } from "../../textEditor/MarkdownEditor";

export function ViewProfile() {
  const [profile] = useState<ViewableDevProfile>();
  return (
    <div className="login-form">
      <section className="form-section">
        <span>Username</span>
        <input
          type="text"
          className="input normal-font input-spacing"
          style={{
            width: "75%",
            backgroundColor: "var(--border-cl)",
            opacity: 0.75,
          }}
          readOnly
          value={profile?.userName}
        />
      </section>
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
          value={profile?.fullName}
        />
      </section>
      <section className="form-section">
        <span>Description</span>
        <div style={{ marginTop: "1em", marginBottom: "3em", width: "75%" }}>
          <MarkdownEditor readOnly={true} markdown={profile?.description} />
        </div>
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
          value={profile?.primaryLangName}
        />
      </section>
      <section className="form-section">
        <span>Secondary Language</span>
        <input
          type="text"
          className="input normal-font input-spacing"
          style={{
            width: "75%",
            backgroundColor: "var(--border-cl)",
            opacity: 0.75,
          }}
          readOnly
          value={profile?.secondaryLangName}
        />
      </section>
    </div>
  );
}
