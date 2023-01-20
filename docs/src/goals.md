# Goal / Scope

__Disclaimer: While `types` are great. This doesn't introduce any types into nix.__

## The Goal / Scope

### Introduce a type annotation system for the doc-strings

Which can be seen as the first step towards static types.

### Enhance the system so it is possible to annotate the type of everything in nix

- Write down *convention* of the current typing system in nix. (`Type:` doc-stings)
- Let the convention be so, that we can parse the `Type:` into an useful `AST` (Abstract Syntax Tree).
- Provide a type-system that could proof correctness of code before runtime.
- Provide a Parser and AST Specification for that type system.

Currently there is the `type:` pattern which can be parsed from nixdoc. Which is a good start but not enough.
The goal is to build more accurate type comments and have a consistent convention on what is actually allowed and what is not.  

Although nix doesn't have a type system, it has types. (See [here](https://nixos.org/manual/nix/stable/language/values.html))

Because there are different ways for developers to express typings they are very inconsistent
