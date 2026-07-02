import { ValidationSeverity, ValidationMessage } from "../types";

interface ValidationResultsProps {
  severity: ValidationSeverity;
  messages: ValidationMessage[];
}

export function ValidationResults({ severity, messages }: ValidationResultsProps) {
  // Determine overall status colors and text
  let statusClass = "valid";
  let statusLabel = "Valid";
  let statusText = "No issues detected. The file is ready for import.";
  
  if (severity === "Warning") {
    statusClass = "warning";
    statusLabel = "Warning";
    statusText = "The file is structurally valid but contains warnings or suspicious values.";
  } else if (severity === "Invalid") {
    statusClass = "invalid";
    statusLabel = "Invalid";
    statusText = "The file contains structural errors and cannot be imported safely.";
  }

  // Count errors and warnings
  const errorCount = messages.filter((m) => m.severity === "Invalid").length;
  const warningCount = messages.filter((m) => m.severity === "Warning").length;

  return (
    <div className="validation-results-card">
      <div className={`validation-status-banner ${statusClass}`}>
        <div className="status-header">
          <span className={`status-dot ${statusClass}`} />
          <h3 className="status-title">Status: {statusLabel}</h3>
        </div>
        <p className="status-description">{statusText}</p>
        <div className="status-counts">
          <span className="count-badge error-badge">
            {errorCount} {errorCount === 1 ? "Error" : "Errors"}
          </span>
          <span className="count-badge warning-badge">
            {warningCount} {warningCount === 1 ? "Warning" : "Warnings"}
          </span>
        </div>
      </div>

      {messages.length > 0 && (
        <div className="validation-messages-list-container">
          <h4 className="list-title">Detailed Validation Log</h4>
          <div className="validation-messages-list">
            {messages.map((msg, index) => (
              <div key={index} className={`validation-message-item ${msg.severity.toLowerCase()}`}>
                <div className="message-meta">
                  <span className={`message-severity-badge ${msg.severity.toLowerCase()}`}>
                    {msg.severity}
                  </span>
                  {msg.row_number !== null && (
                    <span className="message-loc">Row {msg.row_number}</span>
                  )}
                  {msg.column !== null && (
                    <span className="message-loc">Col {msg.column}</span>
                  )}
                </div>
                <div className="message-body">
                  <p className="message-desc">{msg.description}</p>
                  {msg.offending_value !== null && (
                    <div className="message-offending">
                      <span className="offending-label">Offending value:</span>
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
