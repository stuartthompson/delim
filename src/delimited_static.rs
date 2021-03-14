/// Returns a value between matched delimiters from part of a string.
///
/// # Arguments
///
/// * `self` - The delimited string to search.
/// * `delim` - The delimiter to match within.
pub fn matched<T>(s: &str, delim: &str) -> Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    if let Some(s) = matched_s(&s, delim) {
        Some(s.parse::<T>().unwrap())
    } else {
        None
    }
}

/// Returns a substring between enclosing matched delimiters.
///
/// # Arguments
///
/// * `self` - The delimited string to search.
/// * `delim` - The enclosing delimiter.
pub fn matched_s<'a>(s: &'a str, delim: &str) -> Option<&'a str> {
    mismatched_s(&s, delim, delim)
}

/// Returns a value between matched delimiters from part of a string.
///
/// # Arguments
///
/// * `self` - The delimited string to search.
/// * `delim_start` - The opening delimiter.
/// * `delim_end` - The closing delimiter.
pub fn mismatched<T>(s: &str, delim_start: &str, delim_end: &str) -> Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    if let Some(s) = mismatched_s(&s, delim_start, delim_end) {
        Some(s.parse::<T>().unwrap())
    } else {
        None
    }
}

/// Returns a substring between enclosing mismatched delimiters from part of a 
///  string.
///
/// # Arguments
///
/// * `self` - The delimited string to search.
/// * `delim_start` - The opening delimiter.
/// * `delim_end` - The closing delimiter.
pub fn mismatched_s<'a>(
    s: &'a str,
    delim_start: &str,
    delim_end: &str,
) -> Option<&'a str> {
    if let Some(mut s_ix) = &s.find(delim_start) {
        s_ix += 1; // Consume matched leading delim
        if let Some(e_ix) = &s[s_ix..].find(delim_end) {
            Some(&s[s_ix..e_ix + s_ix])
        } else {
            None
        }
    } else {
        None
    }
}

/// Returns a value prefixed by a delimiter.
///
/// # Arguments
///
/// * `self` - The delimited string to search.
/// * `delim` - The delimiter prefix.
/// * `len` - The length of the prefixed value, in bytes.
pub fn prefixed<T>(s: &str, delim: &str, len: usize) -> Option<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    if let Some(s) = prefixed_s(&s, delim, len) {
        Some(s.parse::<T>().unwrap())
    } else {
        None
    }
}

/// Returns a substring of bytes following a prefix delimiter.
///
/// # Arguments
///
/// * `self` - The delimited string to search.
/// * `delim` - The prefix delimiter.
/// * `len` - The length of the prefixed substring, in bytes.
pub fn prefixed_s<'a>(s: &'a str, delim: &str, len: usize) -> Option<&'a str> {
    if let Some(mut s_ix) = &s.find(delim) {
        s_ix += 1; // Consume matched leading delim
        Some(&s[s_ix..s_ix + len])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #region tests: matched

    /// Matched delimiters; happy path
    #[test]
    fn test_matched() {
        assert_eq!(matched("abc:12:def", ":"), Some(12));
    }

    /// Matched delimiters; empty string
    #[test]
    fn test_matched_empty() {
        assert_eq!(matched::<usize>("", ":"), None);
    }

    /// Matched delimiters; start delimiter missing
    #[test]
    fn test_matched_missing_start() {
        assert_eq!(matched::<usize>("abc12:def", ":"), None);
    }

    /// Matched delimiters; end delimiter missing
    #[test]
    fn test_matched_missing_end() {
        assert_eq!(matched::<usize>("abc:12def", ":"), None);
    }

    /// Matched delimiters; both delimiters missing
    #[test]
    fn test_matched_missing_both() {
        assert_eq!(matched::<usize>("abcdef", ":"), None);
    }

    // #endregion tests: matched

    // #region tests: matched_s

    /// Matched delimiters (str); happy path
    #[test]
    fn test_matched_s() {
        assert_eq!(matched_s("abc:12:def", ":"), Some("12"));
    }

    /// Matched delimiters (str); empty string
    #[test]
    fn test_matched_s_empty() {
        assert_eq!(matched_s("", ":"), None);
    }

    /// Matched delimiters (str); start delimiter missing
    #[test]
    fn test_matched_s_missing_start() {
        assert_eq!(matched_s("abc12:def", ":"), None);
    }

    /// Matched delimiters (str); end delimiter missing
    #[test]
    fn test_matched_s_missing_end() {
        assert_eq!(matched_s("abc:12def", ":"), None);
    }

    /// Matched delimiters (str); both delimiters missing
    #[test]
    fn test_matched_s_missing_both() {
        assert_eq!(matched_s("abcdef", ":"), None);
    }

    // #endregion tests: matched_s

    // #region tests: mismatched

    /// Mismatched delimiters; happy path
    #[test]
    fn test_mismatched() {
        assert_eq!(mismatched("abc:12;def", ":", ";"), Some(12));
    }

    /// Mismatched delimiters; empty string
    #[test]
    fn test_mismatched_empty() {
        assert_eq!(mismatched::<usize>("", ":", ";"), None);
    }

    /// Mismatched delimiters; start missing
    #[test]
    fn test_mismatched_missing_start() {
        assert_eq!(mismatched::<usize>("abc12;def", ":", ";"), None)
    }

    /// Mismatched delimiters; end missing
    #[test]
    fn test_mismatched_missing_end() {
        assert_eq!(mismatched::<usize>("abc:12def", ":", ";"), None)
    }

    /// Mismatched delimiters; both missing
    #[test]
    fn test_mismatched_missing_both() {
        assert_eq!(mismatched::<usize>("abc12def", ":", ";"), None)
    }

    // #endregion tests: mismatched

    // #region tests: mismatched_s

    /// Mismatched delimiters (str); happy path
    #[test]
    fn test_mismatched_s() {
        assert_eq!(mismatched_s("abc:12;def", ":", ";"), Some("12"));
    }

    /// Mismatched delimiters (str); empty string
    #[test]
    fn test_mismatched_s_empty() {
        assert_eq!(mismatched_s("", ":", ";"), None);
    }

    /// Mismatched delimiters (str); start missing
    #[test]
    fn test_mismatched_s_missing_start() {
        assert_eq!(mismatched_s("abc12;def", ":", ";"), None)
    }

    /// Mismatched delimiters (str); end missing
    #[test]
    fn test_mismatched_s_missing_end() {
        assert_eq!(mismatched_s("abc:12def", ":", ";"), None)
    }

    /// Mismatched delimiters (str); both missing
    #[test]
    fn test_mismatched_s_missing_both() {
        assert_eq!(mismatched_s("abc12def", ":", ";"), None)
    }

    // #endregion tests: mismatched_s

    // #region tests: prefixed

    /// Prefixed; happy path
    #[test]
    fn test_prefixed() {
        assert_eq!(prefixed("abc<12def", "<", 2), Some(12));
    }

    /// Prefixed; empty string
    #[test]
    fn test_prefixed_empty() {
        assert_eq!(prefixed::<usize>("", "<", 2), None);
    }

    /// Prefixed; start delimiter missing
    #[test]
    fn test_prefixed_missing_start() {
        assert_eq!(prefixed::<usize>("abc12:def", "<", 2), None);
    }

    /// Prefixed; end delimiter missing
    #[test]
    fn test_prefixed_missing_end() {
        assert_eq!(prefixed::<usize>("abc:12def", "<", 2), None);
    }

    /// Prefixed; both delimiters missing
    #[test]
    fn test_prefixed_missing_both() {
        assert_eq!(prefixed::<usize>("abcdef", "<", 2), None);
    }

    // #endregion tests: prefixed

    // #region tests: prefixed_s

    /// Prefixed (str); happy path
    #[test]
    fn test_prefixed_s() {
        assert_eq!(prefixed_s("abc<12def", "<", 2), Some("12"));
    }

    /// Prefixed (str); empty string
    #[test]
    fn test_prefixed_s_empty() {
        assert_eq!(prefixed_s("", "<", 2), None);
    }

    /// Prefixed (str); start delimiter missing
    #[test]
    fn test_prefixed_s_missing_start() {
        assert_eq!(prefixed_s("abc12:def", "<", 2), None);
    }

    /// Prefixed (str); end delimiter missing
    #[test]
    fn test_prefixed_s_missing_end() {
        assert_eq!(prefixed_s("abc:12def", "<", 2), None);
    }

    /// Prefixed (str); both delimiters missing
    #[test]
    fn test_prefixed_s_missing_both() {
        assert_eq!(prefixed_s("abcdef", "<", 2), None);
    }

    // #endregion tests: prefixed_s

}