# Operators

__All Operators SHOULD be used with surrounding whitespace.__

## `::` declares the type

The variable name on the LHS is declared to have the `type` on the RHS

e.g. `name :: String`

## `()` Parenthesis

Parenthesis to clarify order of type evaluation

e.g. `( a -> b ) | Bool`

Precedence: (Highest)

## `,` Separator for subsequent entries (like in AttrSet)

e.g. `{ foo :: Number, bar :: String }`

## `|` syntactic or

syntactic `Or` can be used for composition or enums

Let `T` and `U` be different Types.
Then the `|` operator evaluates to either `T` or `U`.

> Sometimes within this paper `|` is written as `{or}`
>
> This is only due to readability and not allowed in the real language

Precedence: 2

### Examples

`Float | Int`

`( Number | Bool ) | Int`

`{ opt :: Int | String }`

```nix
# lets say we want to create a type which can represent the type of both 'foo' and 'bar'
let
  /*
    Type:
      FooBar = Int | String

      foo :: FooBar
  */
  foo = 1;
  /*
    Type: 
      bar :: FooBar
  */
  bar = "baz";

in
  # ....

```

### `...` - arbitrary input values

can only be used within an AttrSet to allow `Any` more `name-value pairs`.

`...` = `${rest} :: a` within an AttrSet context

e.g.

```nix
/*
  Type: foo :: { bar :: a, ...} -> a
*/
Foo = {bar, ...}@inp:
#...
```

Precedence: None

## `//` merge operator

syntactically `merges` Types of AttrSets

`{ foo :: String } // { bar :: Number }` => `{ foo :: String, bar :: Number }`

`{ foo :: String } // { ${names} :: a }` => `{ foo :: String, ${names} :: a  }`

Overwrites occur like in the nix language itself

`{ foo :: String } // { foo :: Number }` => `{ foo :: Number }`

Precedence: 3

## `->` arrow operator

Allows for lambda types

Precedence: 1

## `++` Concat list types

--e.g.  `[ Int ] ++ [ Float ]`

->  `[ Int | Float ]`

Note: The resulting type is __always__ a union of all list item types.

Precedence: 5

## `?` optional arguments in an AttrSet

--e.g.  `{ opt :: Int ? }`

Note: The __very end__ of `type` side contains the `?` operator.

The reason for this is __extendability__. Initially the `?`-Operator is introduced as Unary-operator, marking the entry type as optional.
In future it will be desirable to use the `?` as binary-operator to add default values on optional entries.

e.g.

```nix
{
  foo :: Int ? 1;
}
```

Precedence: None

## Some more special cases

- Inverted Types
- Omit types
- Pick types

### `!` Invert

Sometimes we don't know what type we have, we just know what we don't have, because we checked it, or filtered sth out.

e.g.

consider a function with the following signature:

A function that takes anything and returns everything but never an Integer type.

```nix
a -> !Int  
```

An attrsets that contains arbitrary keys. The only thing we know, it does not contain entry `foo`

```nix
{
  !foo;
  ...
}
```

We can also leave out the type of foo, because we don't need to specify the value of a key that doesn't exist.

### `.` pick types

consider the following example:

```nix
let 
  set :: {
    foo :: String;
    bar :: String;
    baz :: Float;
    ...
  };
in 
  {
    foo :: set.foo;
    bar :: set.bar;
    bar :: set.baz;
  }
```

This quickly becomes annoying and would be nice to have a simple way of writing
nix as a language offers the `inherit` keyword to do that.

So we propose to follow that approach

```nix
let 
  set :: {
    foo :: String;
    bar :: String;
    baz :: Float;
    ...
  };
in 
  {
    inherit (set) foo bar baz;
  }
```

## Const types

Instead of assigning types it is allowed to assign a nix-value to the type. Which is then declared constant across that specific type.

e.g. `{ foo :: "bar", bar :: 1 }` specifies the name `foo` to be of value "bar"

```nix
{
  foo = "bar";
  bar = 1;
}
```

concrete use-case

```nix
/*
    type:
        derivation :: { 
            type: "derivation",
            ...
        }
*/
derivation = ...
```
