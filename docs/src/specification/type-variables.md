# Type Variables

## Abstract

As the explicit `Any` keyword is prohibited. Even there is no such `Any` type.

It is of course possible to construct the `Any` type and use it properly but in good type systems there is no need for that type.

The `Any` type is covered from `Polymorphism` with the usage of `type variables`.

Type variables can have any name but it is usually considered best practice to choose either letters from the latin alphabet or use meaningful names.

> `Type variables` must be written in LOWERCASE to prevent confusion.

## Examples

The following function signatures from `nix` and `nixpkgs` are written most clearly with `type variables`

### builtins.map

```nix
( a -> b ) -> [a] -> [b]
```

### builtins.catAttrs

```nix
String -> [ { ${name} :: a } ] -> [a]
```

### lib.attrsets.attrByPath

```nix
[ String ] -> a -> { ${path} :: b } -> (a | b)
```

### builtins.mapAttrs

```nix
(String -> a -> b) -> { ${name} :: a } -> { ${name} :: b }
```

### builtins.sort

```nix
(a -> b -> Bool) -> [a] -> [b]
```

## Namespaces

Type variables are currently the best way to properly use polymorphism in type signatures.
But using them requires the underlying parser and the user need to understand some concepts like namespaces and shadowing.

This section orients itself on the *Haskell type variables*. See here for [their rules](https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/scoped_type_variables.html)

> In nix we don't have the same syntax as haskell has but the basic principles and core concept will be adopted from the parser.

### Bigger sets with nested type variables

sometime it is possible to have expressions, that contain multiple levels of nestings, where also different hierarchies of `type variables` must interact with each other.

```nix
{ foo :: a } -> {
  nestedFunction :: a -> b,
  bar :: ( b -> c ) -> [c] -> [b]
}
```

This seem a little complex now, because the parent function returns an AttrSet that includes function signatures using the same `type variables` as the outermost function itself.

> **There is actually a very strict and proven ruleset for that exact problem out there.**
>
> The Haskell Ruleset for [scoped type variables](https://downloads.haskell.org/ghc/latest/docs/users_guide/exts/scoped_type_variables.html)
>
> (see also above).
