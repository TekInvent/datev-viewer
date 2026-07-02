use crate::models::{BookingRecord, Totals, ValidationMessage, ValidationSeverity};

/// Calculates summary totals and counts from parsed records and validation messages.
pub fn calculate_totals(
    records: &[BookingRecord],
    validation_messages: &[ValidationMessage],
) -> Totals {
    let booking_count = records.len();
    let mut total_amount = 0.0;
    let mut debit_total = 0.0;
    let mut credit_total = 0.0;

    for record in records {
        if let Ok(amt) = record.amount.replace(',', ".").parse::<f64>() {
            total_amount += amt;
            if record.debit_credit == "S" {
                debit_total += amt;
            } else if record.debit_credit == "H" {
                credit_total += amt;
            }
        }
    }

    let warning_count = validation_messages
        .iter()
        .filter(|m| m.severity == ValidationSeverity::Warning)
        .count();

    let error_count = validation_messages
        .iter()
        .filter(|m| m.severity == ValidationSeverity::Invalid)
        .count();

    let (debit_res, credit_res) = if booking_count > 0 {
        (
            Some(round_to_2_dec(debit_total)),
            Some(round_to_2_dec(credit_total)),
        )
    } else {
        (None, None)
    };

    Totals {
        booking_count,
        total_amount: round_to_2_dec(total_amount),
        debit_total: debit_res,
        credit_total: credit_res,
        warning_count,
        error_count,
    }
}

fn round_to_2_dec(val: f64) -> f64 {
    (val * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_records() {
        let totals = calculate_totals(&[], &[]);
        assert_eq!(totals.booking_count, 0);
        assert_eq!(totals.total_amount, 0.0);
        assert_eq!(totals.debit_total, None);
        assert_eq!(totals.credit_total, None);
        assert_eq!(totals.warning_count, 0);
        assert_eq!(totals.error_count, 0);
    }

    #[test]
    fn test_sum_and_rounding() {
        let records = vec![
            BookingRecord {
                row_number: 3,
                amount: "100,055".to_string(),
                debit_credit: "S".to_string(),
                date: "0101".to_string(),
                account: "1200".to_string(),
                contra_account: "1600".to_string(),
                tax_key: "".to_string(),
                document_reference: "".to_string(),
                booking_text: "".to_string(),
                raw_fields: vec![],
            },
            BookingRecord {
                row_number: 4,
                amount: "200,104".to_string(),
                debit_credit: "H".to_string(),
                date: "0201".to_string(),
                account: "1400".to_string(),
                contra_account: "8400".to_string(),
                tax_key: "".to_string(),
                document_reference: "".to_string(),
                booking_text: "".to_string(),
                raw_fields: vec![],
            },
            BookingRecord {
                row_number: 5,
                amount: "50,00".to_string(),
                debit_credit: "S".to_string(),
                date: "0301".to_string(),
                account: "1200".to_string(),
                contra_account: "1600".to_string(),
                tax_key: "".to_string(),
                document_reference: "".to_string(),
                booking_text: "".to_string(),
                raw_fields: vec![],
            },
        ];

        let messages = vec![
            ValidationMessage {
                severity: ValidationSeverity::Warning,
                row_number: Some(1),
                column: None,
                description: "warning".to_string(),
                offending_value: None,
            },
            ValidationMessage {
                severity: ValidationSeverity::Invalid,
                row_number: Some(3),
                column: None,
                description: "error".to_string(),
                offending_value: None,
            },
        ];

        let totals = calculate_totals(&records, &messages);

        assert_eq!(totals.booking_count, 3);
        assert_eq!(totals.total_amount, 350.16);
        assert_eq!(totals.debit_total, Some(150.06));
        assert_eq!(totals.credit_total, Some(200.10));
        assert_eq!(totals.warning_count, 1);
        assert_eq!(totals.error_count, 1);
    }
}
