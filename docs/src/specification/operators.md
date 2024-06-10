# Operators

__All Operators SHOULD be used with surrounding whitespace.__

## `::` declares the type

The variable name on the LHS is declared to have the `type` on the RHS

e.g. `name :: String`

In attribute set contexts `::`
defines both the type of `a` and `b` as illustrated below.

```pseudocode
def pair=λa.λb.λf.((f a) b)
<=>
{ "foo" = 1; }

typeOf { "foo" = 1; }
=>
{ String :: Int; } # Example type conversion may not closely represent the type conversion done by a real type engine.

def first p​=λa.λb.a

first { String :: Int; }
=>
String

def second p=λa.λb.b

second { String :: Int; }
=>
Int

```

## `()` Parenthesis

Parenthesis to clarify order of type evaluation

e.g. `( a -> b ) | Bool`

Precedence: (Highest)

## `;` Separator for subsequent entries (like in AttrSet)

e.g. `{ foo :: Number; bar :: String }`

> Important: This is a required syntax rule! It terminates the type expression.
>
> Currently this is very inconsistent in nixpkgs.

## `|` syntactic or

syntactic `Or` can be used for composition or enums

Let `T` and `U` be different Types.
Then the `|` operator evaluates to either `T` or `U`.

> Sometimes within this paper `|` is written as `{or}`
>
> This is only due to readability and not allowed in the real language

Precedence: 2

### Triviality / special Cases

```hs
Any | a
```

Is always Any; Because any other type `a` must already be a subtype of any, due to the definition of `Any`.

```hs
b | Never
```

Is always `b`; Due to the definition of `Never`; `b` must be a supertype of `Never`.

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

## `&` conjunction operator

TODO

### Triviality / special Cases

```hs
Any & a
```

Is always `a`

```hs
b & Never
```

Is always `Never`

## `...` - arbitrary input values

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

`{ foo :: String } // { bar :: Number }` => `{ foo :: String; bar :: Number }`

`{ foo :: String } // { ${names} :: a }` => `{ foo :: String; ${names} :: a  }`

Overwrites occur like in the nix language itself

`{ foo :: String } // { foo :: Number }` => `{ foo :: Number }`

Precedence: 3

## `->` arrow operator

Allows for lambda types

Precedence: 1

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

## Precedence

Precedence is just taken from the official nix operators.
To make usage as intuitive as possible.
However the type language only uses a tight subset of the operator available in nix.

-> see [grammar](./grammar.md)
