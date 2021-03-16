#[derive(Debug)]
pub struct Delimited<'a> {
    s: &'a str,
    ix: usize,
}

impl<'a> Delimited<'a> {
    /// Returns a new delimited string.
    #[must_use]
    pub fn new(
        s: &str
    ) -> Delimited {
        Delimited { s, ix: 0 }
    }

    /// Returns the next value between the supplied matched delimiters.
    ///
    /// Advances the cursor to the end of the matched value, if found.
    ///
    /// # Panics
    ///
    /// If the matched value cannot be parsed to the specified type.
    #[must_use]
    pub fn matched<T>(
        &mut self,
        delim: &str
    ) -> Option<T>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.matched_s(delim).map(|m| m.parse::<T>().unwrap())
    }

    /// Returns the next string value between matched delimiters.
    ///
    /// Advances the cursor to the end of the matched value, if found.
    #[must_use]
    pub fn matched_s(
        &mut self,
        delim: &str
    ) -> Option<&'a str> {
        self.mismatched_s(delim, delim)
    }

    /// Returns the next value between mismatched delimiters.
    ///
    /// Advances the cursor to the end of the matched value, if found.
    ///
    /// # Panics
    ///
    /// If the matched value cannot be parsed to the specified type.
    #[must_use]
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
            .map(|m| m.parse::<T>().unwrap())
    }

    /// Returns the next string value between mismatched delimiters.
    ///
    /// Advances the cursor to the end of the matched value, if found.
    #[must_use]
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
        self.s[self.ix..].find(delim_start)
            .and_then(|s_ix| {
                let from = self.ix + s_ix + 1; // Consume matched leading delim
                self.s[from..].find(delim_end)
                    .and_then(|len| {
                        if len > 0 {
                            self.ix += s_ix + len + 2;
                            Some(&self.s[from..from + len])
                        } else { None }
                })
        })
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
