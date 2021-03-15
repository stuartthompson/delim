# Delim

This crate helps with parsing delimiter-formatted strings.

## Usage Examples

The following shows how to retrieve a value from a string wrapped by matched 
delimiters:
```
let d = Delimited::new("abc:12:def");
assert_eq!(d.matched(":"), Some(12));
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


