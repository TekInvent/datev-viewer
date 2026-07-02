use serde::{Deserialize, Serialize};
use std::path::Path;

/// Supported encoding types for DATEV files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncodingType {
    Utf8,
    Windows1252,
}

impl EncodingType {
    /// Returns the standard string representation of the encoding.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Utf8 => "UTF-8",
            Self::Windows1252 => "Windows-1252",
        }
    }
}

impl std::fmt::Display for EncodingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Supported line ending types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LineEnding {
    CrLf,
    Lf,
}

impl LineEnding {
    /// Returns the standard string representation of the line ending.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CrLf => "CRLF",
            Self::Lf => "LF",
        }
    }
}

impl std::fmt::Display for LineEnding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Represents the decoded file content along with detected metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecodedFile {
    pub content: String,
    pub encoding: EncodingType,
    pub line_ending: LineEnding,
    pub file_size: u64,
}

/// Decodes a byte slice into a string, detecting whether the encoding
/// is UTF-8 or Windows-1252, and detects the line ending type.
pub fn decode_bytes(bytes: &[u8]) -> DecodedFile {
    // 1. Detect encoding (UTF-8 BOM, standard UTF-8, or Windows-1252 fallback)
    let (content, encoding) = if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        let rest = &bytes[3..];
        match std::str::from_utf8(rest) {
            Ok(valid_utf8) => (valid_utf8.to_string(), EncodingType::Utf8),
            Err(_) => {
                let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
                (decoded.into_owned(), EncodingType::Windows1252)
            }
        }
    } else {
        match std::str::from_utf8(bytes) {
            Ok(valid_utf8) => (valid_utf8.to_string(), EncodingType::Utf8),
            Err(_) => {
                let (decoded, _, _) = encoding_rs::WINDOWS_1252.decode(bytes);
                (decoded.into_owned(), EncodingType::Windows1252)
            }
        }
    };

    // 2. Detect line endings
    let line_ending = detect_line_ending(&content);

    DecodedFile {
        content,
        encoding,
        line_ending,
        file_size: bytes.len() as u64,
    }
}

/// Reads a file from disk, detects its encoding, decodes it, and detects line endings.
pub fn read_file<P: AsRef<Path>>(path: P) -> std::io::Result<DecodedFile> {
    let bytes = std::fs::read(path)?;
    Ok(decode_bytes(&bytes))
}

/// Detects the line ending type by finding the first newline character in the content.
fn detect_line_ending(content: &str) -> LineEnding {
    if let Some(pos) = content.find('\n') {
        if pos > 0 && content.as_bytes()[pos - 1] == b'\r' {
            LineEnding::CrLf
        } else {
            LineEnding::Lf
        }
    } else {
        // Default to CRLF for files without line endings (e.g. empty or single line files)
        // since Windows/DATEV files typically use CRLF.
        LineEnding::CrLf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_ascii_utf8() {
        let input = b"Hello World\nLine 2";
        let decoded = decode_bytes(input);
        assert_eq!(decoded.content, "Hello World\nLine 2");
        assert_eq!(decoded.encoding, EncodingType::Utf8);
        assert_eq!(decoded.line_ending, LineEnding::Lf);
    }

    #[test]
    fn test_decode_utf8_with_bom() {
        let input = b"\xEF\xBB\xBFHello \xC3\xA4\xC3\xB6\xC3\xBC\r\n";
        let decoded = decode_bytes(input);
        assert_eq!(decoded.content, "Hello äöü\r\n");
        assert_eq!(decoded.encoding, EncodingType::Utf8);
        assert_eq!(decoded.line_ending, LineEnding::CrLf);
    }

    #[test]
    fn test_decode_utf8_without_bom() {
        let input = b"Hello \xC3\xA4\xC3\xB6\xC3\xBC\r\n";
        let decoded = decode_bytes(input);
        assert_eq!(decoded.content, "Hello äöü\r\n");
        assert_eq!(decoded.encoding, EncodingType::Utf8);
        assert_eq!(decoded.line_ending, LineEnding::CrLf);
    }

    #[test]
    fn test_decode_windows1252() {
        // In Windows-1252: ä is 0xE4, ö is 0xF6, ü is 0xFC
        let input = b"Hello \xE4\xF6\xFC\r\n";
        let decoded = decode_bytes(input);
        assert_eq!(decoded.content, "Hello äöü\r\n");
        assert_eq!(decoded.encoding, EncodingType::Windows1252);
        assert_eq!(decoded.line_ending, LineEnding::CrLf);
    }

    #[test]
    fn test_line_ending_detection() {
        assert_eq!(detect_line_ending("line\n"), LineEnding::Lf);
        assert_eq!(detect_line_ending("line\r\n"), LineEnding::CrLf);
        assert_eq!(detect_line_ending("line\nsecond\r\n"), LineEnding::Lf); // First newline is LF
        assert_eq!(detect_line_ending("line\r\nsecond\n"), LineEnding::CrLf); // First newline is CRLF
        assert_eq!(detect_line_ending("noline"), LineEnding::CrLf); // Default
    }
}
