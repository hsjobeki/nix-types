# Builtins

This section contains the proof that all nix `builtins` can be meaningfully and correctly annotated with the proposed type system.

There are currently **89** builtin functions, that can be discovered with

> run `nix __dump-builtins`

Every builtins signature can be found below (sorted alphabetically)

> Following symbols are used below
>
> - ✅ - Compliant
> - ⚠️ - Under construction.

## Builtins - TODO: The type signatures are not compliant yet

{{#include ../generated/builtin-types.md}}
