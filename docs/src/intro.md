# Introduction

This (Draft/Future) RFC targets the `# Type` field declared in [RFC-145](https://github.com/NixOS/rfcs/pull/145)

## Motivation

By specifying the syntax of the type section within nix this opens a wide range of possibilities.

- consistent function / api documentation.
- Type checking - whether you call a function with the expected types.
- Language Server (LSP) support - display inline or hover type signatures.
- Encourage Modular architectures, with well defined and documented interfaces.
- Static type checking (Partial)

## Detailed implementation

We propose to formulate a syntax for the `# Type` section.
This RFC may be the only valid ruleset that applies to that section.

### No need to use code brackets

using single or triple ``\` (backticks) is not necessary. The complete `# Type` section allows writing one single `type expression`. The syntax of that is specified in the following.

### Builds on top of dynamic types

Type information from comments can; just like in the latest js-docs; be used as a source for checking correctness of code.
We could imagine static type checking working in symbiotic ways with dynamic type checking, such as YANTS (by TVL).

### Syntax follows nix

The syntax is mainly inspired and as close as possible to the nix language. Everyone who knows nix can easily learn the type-language for it.
There are only very few modifications that are self explaining and intuitive. While we could imagine a way more complex language, that allows expressing every last detailed relationship.
We think it is more important to provide a good abstraction and find the right compromise between simplicity and expressiveness.

## Comparison nixOS-modules system

In general developers must break complex parts into modules.
Allowing them to follow the SOLID principles. Where you could write the interface definition and then develop or refactor the module behind that interface.
As long as the interface stays the same or is downwards compatible nothing in your software breaks.
This property is a very essential feature for writing stable and extendable code.

Nix offers nixos-modules to write such modules and interface declarations. However those declarations are boilerplate code that may run during evaluation time.
And also increases complexity as there is a lot going on with `merge behavior` `priority (e.g. mkForce)` and so on.
Also it is not really clear what is actually validated and how much it costs in terms of additional runtime overhead.

With type annotations this can be improved. It is not considered mutually exclusive but rather inclusive to use both worlds together.
Validate at runtime what cannot be known statically (before runtime) and validate statically once you are sure about something in a static context. (This is essential called a 'gradual' type system)
A good type system would offer both and even detect automatically when to run the check.

Thanks to @roberth and @theophane to point me towards this.

## Future Work

- In some fictional Future the syntax could be integrated into the nix language. With optional typings; Just like in Python or Javascript.

- Also Interface documentations could then be generated from the nix language itself, if types where specified.
