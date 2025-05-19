//! # Front-matter extractor
//!
//! > The front matter must be the first thing in the file and must take
//! > the form of valid YAML set between triple-dashed lines.
//!
//! ## References
//!
//! - [Jekyll Front Matter](https://jekyllrb.com/docs/front-matter/)
//! - [Hugo Front Matter](https://gohugo.io/content-management/front-matter/)

use std::io::{self, BufRead, Cursor, Seek};

/// Result type for front matter extraction
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error types for front matter extraction
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Front matter is not properly formatted (missing opening or closing delimiter)
    #[error("invalid front matter format")]
    InvalidFormat,

    /// IO error occurred during reading
    #[error(transparent)]
    IoError(#[from] io::Error),
}

/// Extracts front matter from a reader
///
/// Front matter must be at the beginning of the file and enclosed between
/// triple-dashed lines (`---`). Any leading whitespace (or newlines) are
/// ignored.
///
/// Returns a `Result` containing:
///
/// - An optional buffered reader for the front matter content (`None` if no front matter
///   was found)
/// - The original reader positioned at the beginning of the remaining content
///
/// # Errors
///
/// Returns [`Error::InvalidFormat`] if the front matter is not properly formatted
/// (e.g., missing closing delimiter or has content before the opening delimiter), or
/// [`Error::IoError`] if an IO error occurs during reading or seeking operations.
pub fn extract_front_matter<R>(mut reader: R) -> Result<(Option<Cursor<Vec<u8>>>, R)>
where
    R: BufRead + Seek,
{
    const DELIMITER: &str = "---";

    let initial_position = reader.stream_position()?;

    let mut line_buffer = String::with_capacity(256);

    // Search for the opening delimiter
    // Read lines until we find the opening delimiter (or non-whitespace content)
    loop {
        line_buffer.clear();

        let bytes_read = reader.read_line(&mut line_buffer)?;

        // Empty file check
        if bytes_read == 0 {
            return Ok((None, reader));
        }

        let trimmed = line_buffer.trim();
        if trimmed.is_empty() {
            // Skip empty lines
            continue;
        } else if trimmed.starts_with('#') {
            // Skip YAML comments
            continue;
        } else if trimmed == DELIMITER {
            // Found opening delimiter
            break;
        } else {
            // Found non-delimiter content, no front matter
            reader.seek(io::SeekFrom::Start(initial_position))?;
            return Ok((None, reader));
        }
    }

    // If we reach here, we found an opening delimiter
    // Start collecting front matter
    let mut front_matter = Vec::with_capacity(2048); // 2kb

    // Search for the closing delimiter
    // Read lines until we find the closing delimiter (or EOF)
    loop {
        line_buffer.clear();

        let bytes_read = reader.read_line(&mut line_buffer)?;

        // EOF without closing delimiter
        if bytes_read == 0 {
            reader.seek(io::SeekFrom::Start(initial_position))?;
            return Err(Error::InvalidFormat);
        }

        // Check for closing delimiter
        if line_buffer.trim() == DELIMITER {
            // Found closing delimiter
            return Ok((Some(Cursor::new(front_matter)), reader));
        }

        // Add line to front matter content
        front_matter.extend_from_slice(line_buffer.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use std::io::{self, BufRead, Cursor, Read, Seek};

    use serde_yaml::Value as YamlValue;

    use super::{Error, extract_front_matter};

    #[test]
    fn front_matter_present() {
        //* Given
        let raw_file = indoc::indoc! {
            r#"
            ---
            APP_PORT: 80
            ---
            name: "solo-app"
            services:
              app:
                image: nginx:latest
                ports:
                - "${APP_PORT}:80"
            "#
        };

        let mut reader = Cursor::new(raw_file);

        //* When
        let (front_matter_content, remaining_content) =
            extract_front_matter(&mut reader).expect("Failed to extract front matter");

        //* Then
        let front_matter_reader = front_matter_content.expect("Failed to get front matter reader");
        let front_matter_value: YamlValue =
            serde_yaml::from_reader(front_matter_reader).expect("Failed to parse front matter");

        assert_eq!(front_matter_value["APP_PORT"], 80);

        let remaining_content_value: YamlValue =
            serde_yaml::from_reader(remaining_content).expect("Failed to parse remaining content");

        assert_eq!(remaining_content_value["name"], "solo-app");
        assert_eq!(
            remaining_content_value["services"]["app"]["image"],
            "nginx:latest"
        );
        assert_eq!(
            remaining_content_value["services"]["app"]["ports"][0]
                .as_str()
                .unwrap(),
            "${APP_PORT}:80"
        );
    }

    #[test]
    fn no_front_matter() {
        //* Given
        let raw_file = indoc::indoc! {
            r#"
            name: "solo-app"
            services:
              app:
                image: nginx:latest
            "#
        };

        let mut reader = Cursor::new(raw_file);

        //* When
        let (front_matter_content, remaining_content) =
            extract_front_matter(&mut reader).expect("Failed to extract front matter");

        //* Then
        assert!(front_matter_content.is_none());

        let remaining_content_value: YamlValue =
            serde_yaml::from_reader(remaining_content).expect("Failed to parse remaining content");
        assert_eq!(remaining_content_value["name"], "solo-app");
    }

    #[test]
    fn missing_closing_delimiter() {
        //* Given
        let raw_file = indoc::indoc! {
            r#"
            ---
            ENV_KEY1: "ENV_VALUE1"
            ENV_KEY2: "ENV_VALUE2"
            name: "solo-app"
            "#
        };

        let mut reader = Cursor::new(raw_file);

        //* When
        let result = extract_front_matter(&mut reader);

        //* Then
        let err = result.expect_err("Expected invalid front matter format");
        assert!(matches!(err, Error::InvalidFormat));
    }

    #[test]
    fn empty_file() {
        //* Given
        let raw_file = "";

        let mut reader = Cursor::new(raw_file);

        //* When
        let (front_matter_content, remaining_content) =
            extract_front_matter(&mut reader).expect("Failed to extract front matter");

        //* Then
        assert!(front_matter_content.is_none());

        let remaining_content_value: YamlValue =
            serde_yaml::from_reader(remaining_content).expect("Failed to parse remaining content");
        assert!(remaining_content_value.is_null());
    }

    #[test]
    fn not_the_first_thing_in_the_file() {
        //* Given
        let raw_file = indoc::indoc! {
            r#"
            Some content
            ---
            ENV_KEY1: "ENV_VALUE1"
            ---
            name: "solo-app"
            "#
        };

        let mut reader = Cursor::new(raw_file);

        //* When
        let (front_matter_content, remaining_content) =
            extract_front_matter(&mut reader).expect("Failed to extract front matter");

        //* Then
        assert!(front_matter_content.is_none());

        let res = serde_yaml::from_reader::<_, YamlValue>(remaining_content);
        assert!(
            res.is_err(),
            "Expected an error when parsing an invalid file content"
        );
    }

    #[test]
    fn only_opening_delimiter() {
        //* Given
        let raw_file = "---\n";

        let mut reader = Cursor::new(raw_file);

        //* When
        let result = extract_front_matter(&mut reader);

        //* Then
        let err = result.expect_err("Expected invalid front matter format");
        assert!(matches!(err, Error::InvalidFormat));
    }

    #[test]
    fn io_error() {
        //* Given
        #[derive(Debug)]
        struct ErrorReader;

        impl Read for ErrorReader {
            fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                unreachable!()
            }
        }

        impl Seek for ErrorReader {
            fn seek(&mut self, _pos: io::SeekFrom) -> io::Result<u64> {
                Ok(0)
            }
        }

        impl BufRead for ErrorReader {
            fn fill_buf(&mut self) -> io::Result<&[u8]> {
                Err(io::Error::other("mock error"))
            }

            fn consume(&mut self, _amt: usize) {
                // No-op since we always error on fill_buf
                unreachable!()
            }
        }

        let mut error_reader = ErrorReader;

        //* When
        let result = extract_front_matter(&mut error_reader);

        //* Then
        let err = result.expect_err("Expected an IO error");
        assert!(matches!(err, Error::IoError(_)));
    }

    #[test]
    fn newlines_before_delimiter() {
        //* Given
        let raw_file = indoc::indoc! {
            r#"


            ---
            ENV_KEY1: "ENV_VALUE1"
            ---
            name: "solo-app"
            "#
        };

        let mut reader = Cursor::new(raw_file);

        //* When
        let (front_matter_content, remaining_content) =
            extract_front_matter(&mut reader).expect("Failed to extract front matter");

        //* Then
        let front_matter_reader = front_matter_content.expect("Failed to get front matter reader");
        let front_matter_value: YamlValue =
            serde_yaml::from_reader(front_matter_reader).expect("Failed to parse front matter");
        assert_eq!(front_matter_value["ENV_KEY1"], "ENV_VALUE1");

        let remaining_content_value: YamlValue =
            serde_yaml::from_reader(remaining_content).expect("Failed to parse remaining content");
        assert_eq!(remaining_content_value["name"], "solo-app");
    }

    #[test]
    fn omments_before_delimiter() {
        //* Given
        let raw_file = indoc::indoc! {
            r#"
            # This is a YAML comment
              # An indented comment line that should be ignored
            # --- This looks like a delimiter but it's a comment
            ---
            ENV_KEY1: "ENV_VALUE1"
            ---
            name: "solo-app"
            "#
        };

        let mut reader = Cursor::new(raw_file);

        //* When
        let (front_matter_content, remaining_content) =
            extract_front_matter(&mut reader).expect("Failed to extract front matter");

        //* Then
        let front_matter_reader = front_matter_content.expect("Failed to get front matter reader");
        let front_matter_value: YamlValue =
            serde_yaml::from_reader(front_matter_reader).expect("Failed to parse front matter");
        assert_eq!(front_matter_value["ENV_KEY1"], "ENV_VALUE1");

        let remaining_content_value: YamlValue =
            serde_yaml::from_reader(remaining_content).expect("Failed to parse remaining content");
        assert_eq!(remaining_content_value["name"], "solo-app");
    }
}
