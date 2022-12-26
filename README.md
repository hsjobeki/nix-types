# nix-types

While `types` are great. This doesnt introduce any types into nix.

This project aims to induce some *convention* on the current typing system in nix.

Although nix doesn't have a type system, it has types. (See [here](https://nixos.org/manual/nix/stable/language/values.html))

Because there are different ways for developers to express typings they are very inconsistent.

There are multiple ways to document a type:

- With `Type:` comments.
  - unchecked
  - parsed by custom tooling to generate documentation.

- With `mkOption`.
  - checked
  - only works for nixOS-Modules.

:construction: :construction: Any help welcome! :construction: :construction: 



## Abstract

![type-system](./Types.drawio.svg)

Type systems are good:

- A good type-system can proof correctness of code at compile time.
- Additional benefits through linting, self-documenting, etc.

Thats why I decided to give it a try. At least to clearify the conventions of the current type system.
And introduce a really consistent and reliable `intermediate representation`  of types in nix
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

Consitent format would be:

``` [ String ] ```

So I'd like to enhance the `mkOption` (more explizitly the underlying `types` and `mkOptionType`).

Also writing type comments is very tideous and those comments can drift over time, and at one point they might describe not exctly what is going on.
So enhancing nixos modules and improving documentation system on that `self-documenting` system seems really beneficial to me. 

I'd like to have the same comfort beeing used in nixos modules, as automatic documentation. Plus the convention of abstract types, that can acutally be used in a lot of enhancing tools.

- (Statically) Checking correctness of code
- Linting
- Documentation
- Indexing
- Safe usage of APIs
- etc.

## Convention

The goal: __Make all typings; outputs or doc-strings consistent__

## What needs to change

Until there are real types in nix. Some things need to change.

### There are different names / aliases describing the same type in those two worlds. 

- `Package` vs. `StorePath` vs `Derivation` ?
- `Path` vs `String` (representing a Path)

> Clearify how those types work together and if they are the same, or subsets of each another.

### Replace duplicate types

__I would favor the exiting mkOption types and create consistent outputs of them. Following the existing `convention`__

- `types.str` vs `String`
- `types.unspecified` vs `Any`
- `a`,`b`,`c` vs `Any` ?
- `List` vs `[]`
- `AttrSet` vs `{}`

### Add `lint doc-types` to gh-actions. 

Requires some sort of parser, where everything it doesnt accept is an invalid doc-string

> The Parser could then implement and proof the ruleset and vice versa.

### Add missing types

Maybe we need to compose or create new types

- tbd. ?

## Conventions from the `Type:` doc-strings

1. All notations are `PascalCase`, starting with capital letters.
2. Types MUST be choosen from the existing list. (see [below](#List-of-Types) )
3. Operators MUST be choosen from the existing list. (see [below](#List-of-Operators) )
4. `AttrSet` is an alias for `{ Any }`, same for `List` -> `[ Any ]`
5. Single letters `a`, `b`, `c` are an alias for `Any` (while they carry more informations)
6. AttrSets definitions should include their keys if they dont accept arbitrary values. (optional) `{ key :: Any }`
7. Spaces between Operators (optional)

## List of Types

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

## List of Operators

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


