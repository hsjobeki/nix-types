nix-types
======

This tool is (for now) a proof-of-concept to parse `nix-types` described within this project.

## Comment format

Opening a type block

* `Type:\n` Everything following this line will be parsed as a type signature.
* `Type:` Legacy, deprecated.

Keep in mind everything after the opening will be interpreted as type. If you want to write other text you MUST do that before the type block or in another comment block.

## Adding to variables

Everything can be documented by prefixing them with a comment:

```
/* 
  Type:
    myFunction :: a -> b
 */
myFunction =
    # The thing to do
    thing:
    # How many times to do it
    n: doNTimes n thing
```

## Special thanks to nixdoc

This project uses parts of nixdoc internally and builds on the the existing conventions established from nixdoc.

## Caveats & TODOs

Please check the [issues][] page.

## Building

This project requires a nightly Rust compiler build.

[rnix]: https://gitlab.com/jD91mZM2/rnix
[this Discourse thread]: https://discourse.nixos.org/t/nixpkgs-library-function-documentation-doc-tests/1156
[this example]: https://storage.googleapis.com/files.tazj.in/nixdoc/manual.html#sec-functions-library-strings
[issues]: https://github.com/tazjin/nixdoc/issues
