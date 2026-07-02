import { useLocale } from "../i18n";
import { ValidationSeverity, ValidationMessage } from "../types";

interface ValidationResultsProps {
  severity: ValidationSeverity;
  messages: ValidationMessage[];
}

function messageSeverityLabel(
  severity: ValidationSeverity,
  t: ReturnType<typeof useLocale>["t"],
): string {
  if (severity === "Invalid") {
    return t("validation_invalid_label");
  }
  if (severity === "Warning") {
    return t("validation_warning_label");
  }
  return t("validation_valid_label");
}

export function ValidationResults({ severity, messages }: ValidationResultsProps) {
  const { t, tCount } = useLocale();

  let statusClass = "valid";
  let statusLabel = t("validation_valid_label");
  let statusText = t("validation_valid_text");

  if (severity === "Warning") {
    statusClass = "warning";
    statusLabel = t("validation_warning_label");
    statusText = t("validation_warning_text");
  } else if (severity === "Invalid") {
    statusClass = "invalid";
    statusLabel = t("validation_invalid_label");
    statusText = t("validation_invalid_text");
  }

  const errorCount = messages.filter((m) => m.severity === "Invalid").length;
  const warningCount = messages.filter((m) => m.severity === "Warning").length;

  return (
    <div className="validation-results-card">
      <div className={`validation-status-banner ${statusClass}`}>
        <div className="status-header">
          <span className={`status-dot ${statusClass}`} />
          <h3 className="status-title">
            {t("validation_status_prefix")} {statusLabel}
          </h3>
        </div>
        <p className="status-description">{statusText}</p>
        <div className="status-counts">
          <span className="count-badge error-badge">
            {tCount("validation_errors", errorCount)}
          </span>
          <span className="count-badge warning-badge">
            {tCount("validation_warnings", warningCount)}
          </span>
        </div>
      </div>

      {messages.length > 0 && (
        <div className="validation-messages-list-container">
          <h4 className="list-title">{t("validation_log_title")}</h4>
          <div className="validation-messages-list">
            {messages.map((msg, index) => (
              <div key={index} className={`validation-message-item ${msg.severity.toLowerCase()}`}>
                <div className="message-meta">
                  <span className={`message-severity-badge ${msg.severity.toLowerCase()}`}>
                    {messageSeverityLabel(msg.severity, t)}
                  </span>
                  {msg.row_number !== null && (
                    <span className="message-loc">
                      {t("validation_row")} {msg.row_number}
                    </span>
                  )}
                  {msg.column !== null && (
                    <span className="message-loc">
                      {t("validation_col")} {msg.column}
                    </span>
                  )}
                </div>
                <div className="message-body">
                  <p className="message-desc">{msg.description}</p>
                  {msg.offending_value !== null && (
                    <div className="message-offending">
                      <span className="offending-label">{t("validation_offending_label")}</span>
                      <code className="offending-code">{msg.offending_value}</code>
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
