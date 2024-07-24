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

Example

~~~haskell
( a -> Bool ) -> [ a ] -> Bool

--└────────┘  ↑          ↑
--    |       |          └── final return value is a Bool
--    |       └── The function returns a function that takes a list of type 'a'
--    └── function that takes an 'a' and returns True or False (Boolean)
~~~

> [!TIP]
> Prefer Type variables (`a`, `b`) over using the `Any` keyword.

> [!TIP]
> Prefer using more explicit `{}` or `[]` syntax over vague `List` or `AttrSet` keywords.

### Details

Sometimes we don't care what List or Attribute-set someone passes.
Arbitrary Sets or Lists can be denoted with the `...` operator:

`AttrSet of Any`

~~~haskell

{ ... }

-- ↑
-- └── Attribute Set; where the content is unknown / can be anything.


{ ${a} :: b }
-- ↑
-- └── Above is equivalent in meaning with this explicit expression.
-- └── `a` is a type variable. Is constraint to the `string` type, such that it can be used as an attribute key.
-- └── `b` is a type variable. If constraint in other places it has the `Any` type.

~~~

> [!Note]
> `...` never needs `;` (semicolon) because it is always the last entry.

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

### 🤕 Bad practice: `AttrSet with multiple dynamic names`

> [!WARNING]
> The following should be avoided.
> Consider the following interfaces bad - we're mixing different types of values in a single structure -
> It's like having a drawer with both socks and utensils; it's hard to know what you're grabbing without looking.

~~~haskell
-- Invalid !
foo :: {
  ${name} :: Number;
  ${version} :: Derivation;
}
-- ↑
-- └── there are dynamic entries with different types.
-- The keys of `version` contain a Derivation.
-- The keys of `name` contain a Number.
-- This is ill advised, when accessing the above structure with `foo.bar` what type does it value have?
-- Therefore only one dynamic key can exist. Its type is the Union of all dynamic entries.
foo :: {
  ${nameOrVersion} :: Number | Derivation;
}
~~~

The above type is now demonstrated in a concrete value.

~~~nix
{
  "foo" = 1;
  "1.0.1" = derivation;
  "bar" = 2;
  "1.2.1" = derivation;
}
~~~

### ✔️ Good practice `Type variables instead of Any`

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

### Type driven design with nix -

Sometimes, writing a quick note about what type of information (like a number or text) a part of your code is expecting can be helpful. It's like leaving a little API hint for yourself and others.
Coming back to a certain piece of code after a while you will be very thankful that you documented the APIs and don't need to rethink your entire program.

When you do this, you might notice things you didn't see before.

You might find out that using your code is harder than it should be. Sometimes it can help to realize why code isn't as flexible as intended.
Writing down the types helps to step back and see the big picture, making it easier to spot and fix these issues

The full convention is available as mdbook [here](https://typednix.dev).
