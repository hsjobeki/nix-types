# Usage

The goal was to create a system that can add `type hints` to every expression in nix.

A nix file can contain many possible things.

So what kind of expressions and formats does the `RFC` need to support?

- named function expression.
- anonymous functions.
- named variables.

In any possible way nix has only expressions thus if this project covers all kind of expressions, it covers the complete nix language.

The following can be imported as a function called `foo` as the parent folder is named `foo` and contains a `default.nix`

```nix
# /foo/default.nix

{lib, ...}:
let
    applyIt = fn: fn 10;
#....
in
    act: applyIt act
```
