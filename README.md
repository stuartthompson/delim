# Delim

![<img alt="github" src="https://img.shields.io/badge/github-stuartthompson%2Fdelim-008888?style=for-the-badge" height="16">](https://img.shields.io/badge/github-stuartthompson%2Fdelim-008888?style=for-the-badge)
![<img alt="crates.io" src="https://img.shields.io/crates/v/delim?logo=Rust&style=for-the-badge" height="16">](https://img.shields.io/crates/v/delim?logo=Rust&style=for-the-badge)
![<img alt="last commit" src="https://img.shields.io/github/last-commit/stuartthompson/delim?logo=GitHub&style=for-the-badge" height="16">](https://img.shields.io/github/last-commit/stuartthompson/delim?logo=GitHub&style=for-the-badge)
![<img alt="ci status" src="https://img.shields.io/github/workflow/status/stuartthompson/delim/CI?label=Build&logo=GitHub%20Actions&logoColor=%23ffffff&style=for-the-badge" height="16">](https://img.shields.io/github/workflow/status/stuartthompson/delim/CI?label=Build&logo=GitHub%20Actions&logoColor=%23ffffff&style=for-the-badge)

Helps parse strings that use delimeters to separate values.

## Examples

The following retrieves a value wrapped by matched delimiters:
```
assert_eq!(Delimited::matched("abc:12:def", ":"), Some(12));
```

This retrieves a value wrapped within mismatched patterns:
```
assert_eq!(Delimited::mismatched("abc<<12*&def", "<<", "*&"), Some(12));
```

### Consuming Delimited

The following shows how using a Delimited instance along with consume_matched 
can be used to extract two different delimited values from a string.

```
let mut d = Delimited::new("aaa :12: bbb :34: ccc");

// Consume first value (expecting 12)
let first = d.consume_matched(":");

// Consume second value (expecting 34)
let second = d.consume_matched(":");

assert_eq!(first, Some(12));
assert_eq!(second, Some(34));
```

## Delimited Pattern Types

There are three primary delimited pattern types:

* Matched delimiters
* Mismatched delimiters
* Prefixed

### Matched Delimiters Example

In the following string, the value 12 is wrapped by matched ":" delimiters. 
```
abc:12:def
```

### Mismatched Delimiters Example

In this string, the value 12 is wrapped by mismatched ":" and "<" delimiters.
```
abc:12<def
```

### Prefixed Example

In this string, we decide that the value 12 is prefixed by |.
```
abc|12def

Prefixed delimiter searches requires the value length to be specified.


