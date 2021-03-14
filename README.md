# Delim

This crate helps with parsing delimited from strings.

## Usage Examples

The following shows how to retrieve a value from a string wrapped by matched 
delimiters:
```
let d = Delimited::new("abc:12:def");
assert_eq!(d.matched(":"), Some(12));
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


