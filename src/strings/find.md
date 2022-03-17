
# find text in string from a starting point

## `TL;DR`
the solution is:
```rust,no_run,ignore
let index: usize = string[start..]
    .find(pattern)
    .map(|index| index + start)
    .unwrap();
```
to get a pattern's index from a string from a starting point


## Long Story

suppose you want to find 2 double quotes in a string
but `string.find('"')` will only get the first one

in your mind you want to do this `string.find('"', start=10)`, for example

you want to search from a starting point

well you `cant` do that in rust `(directly)`, we are not in python universe

but there is `workaround`

{{#playground ../../examples/strings_find.rs}}


how about extracting the algorithm into a function ?
{{#playground ../../examples/strings_find_final.rs}}
