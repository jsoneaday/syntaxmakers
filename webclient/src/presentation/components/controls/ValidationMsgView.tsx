interface ValidationMsgViewProps {
  /// message created after validation attempt runs
  validationMessage?: string;
  /// success message if all is well
  successMessage?: string;
}

export function ValidationMsgView({
  validationMessage,
  successMessage,
}: ValidationMsgViewProps) {
  return (
    <section
      className="form-section"
      style={{
        color: validationMessage ? "var(--error-cl)" : "",
      }}
    >
      <span>{validationMessage ? validationMessage : successMessage}</span>
    </section>
  );
}
