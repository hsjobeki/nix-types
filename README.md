# nix-types

While `types` are great. This doesnt introduce any types into nix.

This project aims to induce some *convention* on the current typing system in nix.

Although nix doesn't have a type system, it has types. (See [here](https://nixos.org/manual/nix/stable/language/values.html))

Because there are different ways for developers to express typings they are very inconsitent.

There are multiple ways to document a type:

- With `Type:` comments.
  - unchecked
  - parsed by custom tooling to generate documentation.

- With `mkOption`.
  - checked
  - only works for nixOS-Modules.

## Abstract

Type systems are good:

- A good type-system can proof correctness of code at compile time.
- Additional benefits through linting, self-documenting, etc.

Thats why I decided to give it a try. At least to clearify the conventions of the current type system.

In [nipkgs/lib/*](https://github.com/NixOS/nixpkgs/tree/master/lib) there are some files that contain descriptive type comments.

e.g.

`assertMsg :: Bool -> String -> Bool`

This is a very good, abstract, haskell inspired `type annotation`.

However with `mkOption` there is `nixosOptionsDoc`. Which can be used to generate self describing types from `mkOption` generated `options`

In that world

```listOf str```

evaluates to

``` "list of string" ```

Which is inconsistent with the abstract type annotations, that i like more.

## Convention

The goal: __Make all typings; outputs or doc-strings consistent__

Which will allow for easier indexing and automated processing, such as documentation or linting.

## Types

1. All notations are `PascalCase`, starting with capital letters.

Format: Standard Notation - Name

> Types from lib/types.nix:

- Any - anything
- Raw - raw
- Unspecified - Unspecified
- Bool - boolean
- Number - A number (alias int | float)

    Subtypes:

        between
        nonnegative
        positive

        Float - floating point number

        Int - signed integer
            Subtypes:

                u8
                u16
                u32
                s8
                s16
                s32
                between
                ign
                unsign
                sign
                port - u16 for a port number

- String - string
  
    Subtypes:
  
        NonEmptyStr - non-empty string
        SingleLineStr - single-line string
        StrMatching - string matching a pattern

        SeparatedString - strings concatenated with a seperator
            Subtypes:
                lines - string seperated by `\n`
                commas - string seperated by `,`
                envVar - string seperated by `:` 
        PasswdEntry - subclass of `String` not containing newlines or colons.

- AttrSet - attribute set
  
    Subtypes:

        AttrsOf - attribute set of elements
        lazyAttrsOf - lazy attribute set of elements

- Package - derivations or attribute sets with an `outPath` or `__toString` or StorePath

    Subtypes:

        shellPackage - a package with `shellPath` attribute

- Path - path to location (relative or absolute)
- ListOf - list of elements

    Subtypes:

        NonEmptyListOf - list containing at least one element

- Null - null
- Function - function that evaluates to a specific result
- Submodule - A nixos submodule

    Subtypes:

        DeferredModule - A module to be imported in some other part of the configuration
        DeferredModuleWith - A module to be imported in some other part of the configuration
        SubmoduleWith -  A module with specific attributes

- Enum - A value from a set of allowed ones
- OneOf - Any of the types in the given list
- Either - Either value of type `t1` or `t2`
- OptionType - Type of an option type
- CoercedTo - Either value of type `coercedType` or `finalType`
- Unique - Value of given type but with no merging  (i.e. `uniq list`s are not concatenated)

Operators

__All Operators SHOULD be used with surrounding whitespaces.__

__Existing ones.__

- `::`  type-name seperator.

e.g. `name :: Any`

- `->` Function

e.g. `function :: Any -> Any`

- `()` Parenthesis (not a type itself, only for syntatic grouping)

e.g. `( a -> b ) -> c`

- `,` Seperator for subseqeuent entries (like in AttrSet)

e.g. `{ a :: Any, b :: Any }`

- `[]` List

e.g. `[ Any ]`

- `{}` AttrSet

e.g. `{ key :: Any }`

__Missing / Introduced with this Idea.__

- `?` optional arguments in an AttrSet.

e.g.  `{ opt :: ? Int }`

- `|` syntactic `Or` can be used for: `Enum`, `OneOf`, `Either`

e.g.  `{ opt :: Int | String | Path }`


Note: The `type` side contains the `?` operator.

:construction:
