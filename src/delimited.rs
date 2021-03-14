#[derive(Debug)]
pub struct Delimited<'a> {
    s: &'a str,
    ix: usize,
}

impl<'a> Delimited<'a> {
    /// Returns a new delimited string.
    pub fn new(
        s: &str
    ) -> Delimited {
        Delimited { s, ix: 0 }
    }

    pub fn consume_matched<T>(
        &mut self,
        delim: &str
    ) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        if let Some(m) = self.consume_matched_s(delim) {
            Some(m.parse::<T>().unwrap())
        } else {
            None
        }
    }

    pub fn consume_matched_s(
        &mut self,
        delim: &str
    ) -> Option<&'a str> {
        self.consume_mismatched_s(delim, delim)
    }

    pub fn consume_mismatched<T>(
        &mut self,
        delim_start: &str,
        delim_end: &str
    ) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        if let Some(s) = self.consume_mismatched_s(delim_start, delim_end) {
            Some(s.parse::<T>().unwrap())
        } else {
            None
        }
    }

    pub fn consume_mismatched_s(
        &mut self,
        delim_start: &str,
        delim_end: &str
    ) -> Option<&'a str> 
    {
        self.consume_delimited(delim_start, delim_end)
    }

    fn consume_delimited(
        &mut self,
        delim_start: &str,
        delim_end: &str
    ) -> Option<&'a str> 
    {
        if let Some(s_ix) = &self.s[self.ix..].find(delim_start) {
            let from = self.ix + s_ix + 1; // Consume matched leading delim
            if let Some(len) = &self.s[from..].find(delim_end) {
                if *len > 0 {
                    self.ix += s_ix + len + 2;
                    Some(&self.s[from..from + len])
                } else { None }
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #region tests: matched

    #[test]
    fn consume_matched() {
        let mut d = Delimited::new("aaa :12: bbb :34: ccc");

        // Consume first value (expecting 12)
        let first = d.consume_matched(":");

        // Consume second value (expecting 34)
        let second = d.consume_matched(":");

        assert_eq!(first, Some(12));
        assert_eq!(second, Some(34));
    }

    #[test]
    fn consume_matched_empty() {
        let mut d = Delimited::new("");

        assert_eq!(d.consume_matched::<usize>(":"), None);
    }

    #[test]
    fn consume_matched_missing_start() {
        let mut d = Delimited::new("abc:12def");

        assert_eq!(d.consume_matched::<usize>(":"), None);
    }

    #[test]
    fn consume_matched_missing_end() {
        let mut d = Delimited::new("abc12:def");

        assert_eq!(d.consume_matched::<usize>(":"), None);
    }

    #[test]
    fn consume_matched_missing_both() {
        let mut d = Delimited::new("abc12def");

        assert_eq!(d.consume_matched::<usize>(":"), None);
    }

    #[test]
    fn consume_matched_uneven_delimiters() {
        let mut d = Delimited::new("abc:12:def:34ghi");

        assert_eq!(d.consume_matched(":"), Some(12));
        assert_eq!(d.consume_matched::<usize>(":"), None);
    }

    #[test]
    fn consume_matched_empty_delimited_value() {
        let mut d = Delimited::new("abc::def");

        assert_eq!(d.consume_matched::<usize>(":"), None);
    }

    // #endregion tests: matched

    // #region tests: delimited

    #[test]
    fn consume_delimited() {
        let mut d = Delimited::new("abc:12;def");
        
        assert_eq!(d.consume_delimited(":", ";"), Some("12"));
    }

    // #endregion tests: delimited

}