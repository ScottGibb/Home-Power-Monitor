#[derive(Debug, PartialEq)]
pub enum CenterError {
    TooWide { string_len: usize, width: usize },
}

impl std::fmt::Display for CenterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CenterError::TooWide { string_len, width } => {
                write!(f, "String too wide to fit ({} > {})", string_len, width)
            }
        }
    }
}

impl std::error::Error for CenterError {}

pub fn center_string(s: &str, width: usize) -> Result<String, CenterError> {
    if s.len() > width {
        return Err(CenterError::TooWide {
            string_len: s.len(),
            width,
        });
    }
    let left = width.saturating_sub(s.len()) / 2;
    let right = width.saturating_sub(s.len()) - left;
    Ok(format!("{}{}{}", " ".repeat(left), s, " ".repeat(right)))
}

pub fn string_to_char_array<const N: usize>(s: &str) -> [char; N] {
    let mut arr = [' '; N];
    for (i, c) in s.chars().take(N).enumerate() {
        arr[i] = c;
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_center_even_width() {
        let s = "abc";
        let result = center_string(s, 8).unwrap();
        assert_eq!(result, "  abc   "); // 2 left, 3 right
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_center_odd_width() {
        let s = "abc";
        let result = center_string(s, 7).unwrap();
        assert_eq!(result, "  abc  "); // 2 left, 2 right
        assert_eq!(result.len(), 7);
    }

    #[test]
    fn test_center_exact_width() {
        let s = "abcdef";
        let result = center_string(s, 6).unwrap();
        assert_eq!(result, "abcdef");
    }

    #[test]
    fn test_center_smaller_width() {
        let s = "abcdef";
        let result = center_string(s, 3);
        assert_eq!(
            result,
            Err(super::CenterError::TooWide {
                string_len: 6,
                width: 3
            })
        );
    }

    #[test]
    fn test_center_empty_string() {
        let s = "";
        let result = center_string(s, 4).unwrap();
        assert_eq!(result, "    ");
    }

    #[test]
    fn test_error_too_wide() {
        let s = "abcdefg";
        let result = center_string(s, 5);
        assert_eq!(
            result,
            Err(super::CenterError::TooWide {
                string_len: 7,
                width: 5
            })
        );
    }
}
