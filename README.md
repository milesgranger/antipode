# antipode

[![CI](https://github.com/milesgranger/antipode/workflows/CI/badge.svg?branch=master)](https://github.com/milesgranger/antipode/actions?query=branch=master)

Calculate the point on earth's surface diametrically opposite to it.

...maybe simplest crate ever? ;-)

Example
-------
```rust
use antipode::antipode;

let coord = (60.394306,  5.325919); // Bergen, Norway
let expected = (-60.394306, -174.674081);  // Somewhere off the coast of New Zealand
assert_eq!(expected, antipode(coord));
```