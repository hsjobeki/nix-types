<!-- markdownlint-disable MD013 -->
# nix-types (the inofficial convention)

This Guide will teach you how to read and write function typing in nix comments.

This typing convention is followed by many nixpkgs functions including those in `nixpgks/lib`

__:construction: :construction: Any help is welcome! :construction: :construction:__

<div align="center">
  <br/>
  Start problem-solving for Nix with types
  <br/>
 Contribute to the | <a href="https://typednix.dev/">Specification</a>
 <br/>
  discussion on | <a href="https://matrix.to/#/#nix-types:matrix.org">matrix<a>
  <br/>
  <br/>
</div>

## The syntax

~~~haskell

( a -> Bool ) -> [ a ] -> Bool

--└────────┘  ↑          ↑           
--    |       |          └── final return value is a Bool
--    |       └── The function returns a function that takes a list of type 'a'      
--    └── function that takes an 'a' and returns True or False (Boolean)
~~~

> Note: We don't use the `Any` type. Type variables carry more information.
>
> __We don't use `List` or `AttrSet`__

### Some more examples

Sometimes we don't care what List or Attribute-set someone passes.
Arbitrary Sets or Lists can be denoted with the `...` operator:

`AttrSet of Any`

~~~haskell

{ ... }

-- ↑           
-- └── Attribute Set; where the content is unknown / can be anything.
~~~

> Note: ellipsis doesn't need `;` semicolon as it is always the last entry.

`List of Any`

~~~haskell

[ ... ]

-- ↑           
-- └── List; where the content is unknown / can be anything.
~~~

`AttrSet with dynamic names`

~~~haskell

{ ${name} :: Bool; }
-- ↑           
-- └── name is of type [ String ]; This list of actual members is calculated at evaluation time. But we know every member has a value of type `Bool` 
~~~

`AttrSet with multiple dynamic names`

~~~haskell

{ 
  ${name} :: Number; 
  ${version} :: Derivation;
}
-- ↑           
-- └── there are multiple dynamic entries. The keys of `version` mapping to a Derivation each.
~~~

~~~nix
{
  "foo" = 1;
  "1.0.1" = derivation;
  "bar" = 2;
  "1.2.1" = derivation;
}
~~~

> As one can see; This is confusing, but this is already widely established in nixpkgs.

`Type variables instead of Any`

~~~haskell
Any -> Any
-- becomes
a -> a
-- ↑           
-- └── This is more concrete. And shows if values relate to each other

-- More 

a -> b -> b
-- e.g. builtins.trace

(a -> b -> c) -> (b -> a -> c)
-- e.g. lib.flip
~~~

### The standard for current doc-strings

A very large set of doc-strings is already available in `nixpkgs`.

The full content is available as mdbook [here](https://typednix.dev).

The current documentation as well as some other external tools have generated content from that inline comments. Thus it is highly desirable to standardize and enhance them.
