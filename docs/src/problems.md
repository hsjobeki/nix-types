# Inconsistencies

When ready through **nixpkgs** there are many comments that document functions and variables like this:

```nix
  /* Construct a Unix-style, colon-separated search path consisting of
     the given `subDir` appended to each of the given paths.
     Type: makeSearchPath :: string -> [string] -> string
     Example:
       makeSearchPath "bin" ["/root" "/usr" "/usr/local"]
       => "/root/bin:/usr/bin:/usr/local/bin"
       makeSearchPath "bin" [""]
       => "/bin"
  */
  makeSearchPath = ...
```

```txt
The premise is that in Nix, every expression evaluates into a value. And every value has a type. And every type can be expressed in code, if we come up with the syntax to do that. 
```

Unfortunately those comments are very inconsistent.

And cause problems if you want to process them with **external tools** or even try to **parse**
them into an Type-AST which could be used **for linting** and other possible features like
**autocompletion**, **typed documentation** or other use-cases of **static code analysis**.

> For simplicity `Type:` prefixes have been omitted. in the following examples

## Type variables vs explicit `Any`

- `singleton :: a -> [a]`
- `singleton :: Any -> [Any]`
- `imap0 :: (int -> a -> b) -> [a] -> [b]`

## Lowercase, PascalCase, vs camelCase

- `imap0 :: (int -> a -> b) -> [a] -> [b]`
- `hasAttrByPath :: [String] -> AttrSet -> Bool`

## What types are there

What type is `ComparableVal`?

- `assertOneOf :: String -> ComparableVal -> List ComparableVal -> Bool`

What type is `package`?

- `string -> string -> [package] -> string`

what type is a `Derivation`?
Is it different from `package`?

- `toDerivation :: Path -> Derivation`

also there is `derivation` which is written lowercase, is that a typo, or the same type?

## Are function names required?

nixdoc by default maps the `Type:` comment to the corresponding function.
Sometimes the type-comment didn't include the functions name.

- `toDerivation :: Path -> Derivation`
- `makeSearchPathOutput :: toDerivation :: Path -> Derivation`

## Should types start in newlines?

```nix
Type: assertOneOf :: String -> ComparableVal -> List ComparableVal -> Bool
```

```nix
Type:
    assertOneOf :: String -> ComparableVal -> List ComparableVal -> Bool
```

## Many more

All those different subjective opinions on how to write proper docstrings have already let to high inconsistency across **nixpkgs**.

## Possible Impact

This project aims to reduce those and provide a consistent ruleset for that.

If it accurately represents the code problems accurate enough,
it might even be valid candidate for static type systems in future nix language versions.
