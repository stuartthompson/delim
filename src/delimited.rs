#[derive(Debug)]
pub struct Delimited<'a> {
    s: &'a str,
    ix: usize,
}

impl<'a> Delimited<'a> {
    #[must_use]
    /// Returns a new delimited string.
    pub fn new(
        s: &str
    ) -> Delimited {
        Delimited { s, ix: 0 }
    }

    /// Consumes a value between matched delimiter patterns.
    ///
    /// This function advances the cursor of the Delimited to the end of the 
    ///  matched value if the specified delimiter pattern was found.
    ///
    /// # Panics
    ///
    /// This function will panic if the value cannot be parsed to the 
    ///  specified type.
    pub fn matched<T>(
        &mut self,
        delim: &str
    ) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.matched_s(delim)
            .map_or(None, |m| Some(m.parse::<T>().unwrap()))
    }

    pub fn matched_s(
        &mut self,
        delim: &str
    ) -> Option<&'a str> {
        self.mismatched_s(delim, delim)
    }

    /// Returns a value between mismatched delimiter patterns.
    ///
    /// This function advances the cursor of the Delimited to the end of the 
    ///  matched value if the specified delimiter pattern was found.
    ///
    ///  # Panics
    ///
    ///  This function will panic if a matched value cannot be parsed to the 
    ///   specified type.
    pub fn mismatched<T>(
        &mut self,
        delim_start: &str,
        delim_end: &str
    ) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.mismatched_s(delim_start, delim_end)
            .map_or(None, |m| Some(m.parse::<T>().unwrap()))
    }

    pub fn mismatched_s(
        &mut self,
        delim_start: &str,
        delim_end: &str
    ) -> Option<&'a str> 
    {
        self.delimited(delim_start, delim_end)
    }

    fn delimited(
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
    fn matched() {
        let mut d = Delimited::new("aaa :12: bbb :34: ccc");

        // Consume first value (expecting 12)
        let first = d.matched(":");

        // Consume second value (expecting 34)
        let second = d.matched(":");

        assert_eq!(first, Some(12));
        assert_eq!(second, Some(34));
    }

    #[test]
    fn matched_empty() {
        let mut d = Delimited::new("");

        assert_eq!(d.matched::<usize>(":"), None);
    }

    #[test]
    fn matched_missing_start() {
        let mut d = Delimited::new("abc:12def");

        assert_eq!(d.matched::<usize>(":"), None);
    }

    #[test]
    fn matched_missing_end() {
        let mut d = Delimited::new("abc12:def");

        assert_eq!(d.matched::<usize>(":"), None);
    }

    #[test]
    fn matched_missing_both() {
        let mut d = Delimited::new("abc12def");

        assert_eq!(d.matched::<usize>(":"), None);
    }

    #[test]
    fn matched_uneven_delimiters() {
        let mut d = Delimited::new("abc:12:def:34ghi");

        assert_eq!(d.matched(":"), Some(12));
        assert_eq!(d.matched::<usize>(":"), None);
    }

    #[test]
    fn matched_empty_delimited_value() {
        let mut d = Delimited::new("abc::def");

        assert_eq!(d.matched::<usize>(":"), None);
    }

    // #endregion tests: matched

    // #region tests: mismatched

    #[test]
    fn mismatched() {
        let mut d = Delimited::new("aaa :12; bbb +34| ccc");

        // Consume first value (expecting 12)
        let first = d.mismatched(":", ";");

        // Consume second value (expecting 34)
        let second = d.mismatched("+", "|");

        assert_eq!(first, Some(12));
        assert_eq!(second, Some(34));
    }

    // #endregion tests: mismatched

    // #region tests: delimited

    #[test]
    fn delimited() {
        let mut d = Delimited::new("abc:12;def");
        
        assert_eq!(d.delimited(":", ";"), Some("12"));
    }

    // #endregion tests: delimited

}
