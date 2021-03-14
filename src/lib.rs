#[allow(unused_macros)]
#[macro_export]
macro_rules! delim {
    ( $s:expr ) => { Delimited::new($s) }
}

#[derive(Debug)]
struct Delimited<'a> {
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
        delim: &str,
    ) -> Option<T> 
        where T: std::str::FromStr, T::Err: std::fmt::Debug
    {
        println!("[Consume matched] s: {} ix: {}", &self.s, self.ix);

        if let Some(s) = Delimited::matched_substr_from(&self.s, delim, self.ix) {
            self.ix += s.find(delim).unwrap() + s.len(); // Advance cursor
            let r = Some(s.parse::<T>().unwrap());
            r
        } else { None }
    }

    pub fn consume_matched_s(
        &'a mut self,
        delim: &str
    ) -> Option<&'a str> {
        let r = 
            Delimited::matched_substr_from(&self.s, delim, self.ix);
        if let Some(s) = r { self.ix += s.len(); } // Advance cursor
        r
    }

    pub fn consume_mismatched<T>(
        &'a mut self,
        delim_start: &str,
        delim_end: &str
    ) -> Option<T> 
        where T: std::str::FromStr, T::Err: std::fmt::Debug
    {
        if let Some(s) = Delimited::mismatched_substr_from(&self.s, delim_start, delim_end, self.ix) {
            self.ix += s.len(); // Advance cursor
            let r = Some(s.parse::<T>().unwrap());
            r
        } else { None }
    }

    pub fn consume_mismatched_s(
        &'a mut self,
        delim_start: &str,
        delim_end: &str
    ) -> Option<&'a str> {
        let r = 
            Delimited::mismatched_substr_from(&self.s, delim_start, delim_end, self.ix);
        if let Some(s) = r { self.ix += s.len(); } // Advance cursor
        r
    }

    /// Returns a value following a prefixed delimiter from the current cursor  
    ///  location within a delimited string.
    /// 
    /// Advances the cursor by the number of bytes consumed.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to consume from.
    /// * `delim` - The prefix delimiter pattern.
    /// * `len` - The number of bytes to consume.
    pub fn consume_prefixed<T>(
        &mut self,
        delim: &str,
        len: usize,
    ) -> Option<T> 
        where T: std::str::FromStr, T::Err: std::fmt::Debug
    {
        self.ix += len; // Advance cursor
        Delimited::prefixed_from(&self.s, delim, len, self.ix)
    }

    /// Returns a substring of bytes following a prefixed delimiter from the 
    ///  current cursor location within a delimited string.
    /// 
    /// Advances the cursor by the number of bytes consumed.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to consume from.
    /// * `delim` - The prefix delimiter pattern.
    /// * `len` - The number of bytes to consume.
    pub fn consume_prefixed_s(
        &mut self,
        delim: &str,
        len: usize,
    ) -> Option<&str> {
        self.ix += len; // Advance cursor
        Delimited::prefixed_substr_from(&self.s, delim, len, self.ix)
    }

    /// Returns a value between matched delimiters.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The delimiter to match within.
    pub fn matched<T>(
        s: &str,
        delim: &str,
    ) -> Option<T>
        where T: std::str::FromStr, 
              T::Err: std::fmt::Debug {
        Delimited::matched_from(&s, delim, 0)
    }

    /// Returns a value between matched delimiters from part of a string.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The delimiter to match within.
    pub fn matched_from<T>(
        s: &str,
        delim: &str,
        from_ix: usize,
    ) -> Option<T>
        where T: std::str::FromStr, 
              T::Err: std::fmt::Debug {
        if let Some(s) = Delimited::matched_substr_from(&s, delim, 0) {
            Some(s.parse::<T>().unwrap())
        } else { None }
    }

    /// Returns a substring between enclosing matched delimiters.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The enclosing delimiter.
    pub fn matched_substr(
        s: &'a str,
        delim: &str,
    ) -> Option<&'a str> {
        Delimited::mismatched_substr(&s, delim, delim)
    }

    /// Returns a substring between enclosing matched delimiters.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The enclosing delimiter.
    /// * `from_ix` - The index to search from.
    pub fn matched_substr_from(
        s: &'a str,
        delim: &str,
        from_ix: usize,
    ) -> Option<&'a str> {
        Delimited::mismatched_substr_from(&s, delim, delim, from_ix)
    }

    /// Returns a value between matched delimiters.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim_start` - The opening delimiter.
    /// * `delim_end` - The closing delimiter.
    pub fn mismatched<T>(
        s: &str,
        delim_start: &str,
        delim_end: &str,
    ) -> Option<T>
        where T: std::str::FromStr, 
              T::Err: std::fmt::Debug {
        Delimited::mismatched_from(&s, delim_start, delim_end, 0)
    }

    /// Returns a value between matched delimiters from part of a string.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim_start` - The opening delimiter.
    /// * `delim_end` - The closing delimiter.
    /// * `from_ix` - The index to start searching from.
    pub fn mismatched_from<T>(
        s: &str,
        delim_start: &str,
        delim_end: &str,
        from_ix: usize,
    ) -> Option<T>
        where T: std::str::FromStr, 
              T::Err: std::fmt::Debug {
        if let Some(s) = Delimited::mismatched_substr_from(&s, delim_start, delim_end, from_ix) {
            Some(s.parse::<T>().unwrap())
        } else { None }
    }

    /// Returns a substring between enclosing mismatched delimiters.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim_start` - The opening delimiter.
    /// * `delim_end` - The closing delimiter.
    fn mismatched_substr(
        s: &'a str,
        delim_start: &str,
        delim_end: &str
    ) -> Option<&'a str> {
       Delimited::mismatched_substr_from(&s, delim_start, delim_end, 0) 
    }

    /// Returns a substring between enclosing mismatched delimiters from part 
    ///  of a string.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim_start` - The opening delimiter.
    /// * `delim_end` - The closing delimiter.
    /// * `from_ix` - The index to start searching from.
    pub fn mismatched_substr_from(
        s: &'a str,
        delim_start: &str,
        delim_end: &str,
        from_ix: usize,
    ) -> Option<&'a str> {
        if let Some(mut s_ix) = &s[from_ix..].find(delim_start) {
            s_ix += from_ix + 1; // Consume matched leading delim
            if let Some(e_ix) = &s[s_ix..].find(delim_end) {
                Some(&s[s_ix..e_ix + s_ix])
            }
            else { None }
        } else { None }
    }

    /// Returns a value prefixed by a delimiter.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The delimiter prefix.
    /// * `len` - The length of the prefixed value, in bytes.
    pub fn prefixed<T>(
        s: &str,
        delim: &str,
        len: usize
    ) -> Option<T> 
        where T: std::str::FromStr,
              T::Err: std::fmt::Debug {
        Delimited::prefixed_from(&s, delim, len, 0)
    }

    /// Returns a value prefixed by a delimiter from part of a string.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The delimiter prefix.
    /// * `len` - The length of the prefixed value, in bytes.
    /// * `from_ix` - The index to search from.
    pub fn prefixed_from<T>(
        s: &str,
        delim: &str,
        len: usize,
        from_ix: usize,
    ) -> Option<T> 
        where T: std::str::FromStr,
              T::Err: std::fmt::Debug {
        if let Some(s) = Delimited::prefixed_substr_from(&s, delim, len, from_ix) {
            Some(s.parse::<T>().unwrap())
        } else { None }
    }

    /// Returns a substring of bytes following a prefix delimiter.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The prefix delimiter.
    /// * `len` - The length of the prefixed substring, in bytes.
    pub fn prefixed_substr(
        s: &'a str,
        delim: &str,
        len: usize,
    ) -> Option<&'a str> {
        Delimited::prefixed_substr_from(&s, delim, len, 0)
    }

    /// Returns a substring of bytes following a prefix delimiter from part of 
    ///  a string.
    /// 
    /// # Arguments
    /// 
    /// * `self` - The delimited string to search.
    /// * `delim` - The prefix delimiter.
    /// * `len` - The length of the prefixed substring, in bytes.
    /// * `from_ix` - The index to start searching from.
    fn prefixed_substr_from(
        s: &'a str,
        delim: &str,
        len: usize,
        from_ix: usize,
    ) -> Option<&'a str> {
        if let Some(mut s_ix) = &s[from_ix..].find(delim) {
            s_ix += from_ix + 1; // Consume matched leading delim
            Some(&s[s_ix..s_ix+len])
        } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_matched() {
        // String with two matches delimited values between : :
        let mut d = Delimited::new("abc:12:def:24:ghi");

        // Consume first value
        let first = d.consume_matched::<usize>(":");
        let second = d.consume_matched::<usize>(":");

        assert_eq!(first, Some(12));
        assert_eq!(second, Some(24));
    }

    #[test]
    fn matched() {
        assert_eq!(
            Delimited::matched("abc:12:def", ":"),
            Some(12)
        );
    }

    #[test]
    fn delim() {
        let s = "abc:12:def";
        let d = delim!(s);

        let expected = Delimited::new("abc:12:def");

        assert_eq!(
            format!("{:?}", d),
            format!("{:?}", expected)
        );
    }

    #[test]
    fn delim_from_literal() {
        let d = delim!("abc:12:def");

        let expected = Delimited::new("abc:12:def");

        assert_eq!(
            format!("{:?}", d),
            format!("{:?}", expected)
        );
    }
}
