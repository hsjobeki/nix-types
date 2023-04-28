# Formals

## As the annotation references to exactly one expression. There is only one type expression possible

## All notations are PascalCase

PascalCase with a starting uppercase letter. This is also in other languages a known convention for `types` or `classes`

## Types MUST be chosen from the existing list

 see [primitives](./basic.md)

## Operators MUST be chosen from the existing list

 see [operators](./operators.md)

## explicit `AttrSet` and `List` keywords are PROHIBITED

See the correct usage [here](./complex.md)

## [complex types](./complex.md) MUST include their members explicitly

There is no `AttrSet` and no `List` keyword.

## explicit `Any` is PROHIBITED

Correct usage with type variables see [here](./type-variables.md)

## Allowing arbitrary values within [complex-types](./complex.md)

It is possible to allow arbitrary values within complex types, but it requires an explicit statement.

> There is no implicit any.

## `Type variables` must be written in LOWERCASE to prevent confusion

They are used instead of the `Any` Keyword.

## For [Type bindings](./operators.md#type-binding) it is PROHIBITED to choose names from the [reserved list](./types.md)

This will help to keep the code clean. Reduces complexity, resolves for shadowing and namespace conflicts, with global names.

## [Optional] Spaces between Operators are recommended

This follows the nix syntax rules. The symbols and grammar rules of the nix language are adopted.

> Spaces are required where they would be required in valid nix syntax.
> This will be quite intuitive
