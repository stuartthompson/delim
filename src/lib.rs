mod delimited;
mod delimited_static;

pub use delimited::Delimited;
pub use delimited_static::*;

#[allow(unused_macros)]
#[macro_export]
macro_rules! delim {
    ( $s:expr ) => { Delimited::new($s) }
}

#[cfg(test)]
mod tests {
    use super::*;
   
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
