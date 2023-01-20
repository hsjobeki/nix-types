# Operators

__All Operators SHOULD be used with surrounding whitespace.__

## `::` declares the type

The variable name on the LHS is declared to have the `type` on the RHS

e.g. `name :: Any`

## `()` Parenthesis

Parenthesis to clarify order of type evaluation

e.g. `( a -> b ) | Bool`

## `,` Separator for subsequent entries (like in AttrSet)

e.g. `{ foo :: Any, bar :: Any }`

## `|` syntactic or

syntactic `Or` can be used for composition or enums

Let `T` and `U` be different Types.
Then the `|` operator evaluates to either `T` or `U`.

> Sometimes within this paper `|` is written as `{or}`
>
> This is only due to readability and not allowed in the real language

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

`...` = `${rest} :: Any` within an AttrSet context

e.g.

```nix
/*
  Type: foo :: { bar :: Any, ...} -> Any
*/
Foo = {bar, ...}@inp:
#...
```

## `//` merge operator

syntactically `merges` Types of AttrSets

`{ foo :: String } // { bar :: Any }` => `{ foo :: String, bar :: Any }`

`{ foo :: String } // { ${names} :: Any }` => `{ foo :: String, ${names} :: Any  }`

Overwrites occur like in the nix language itself

`{ foo :: String } // { foo :: Any }` => `{ foo :: Any }`

## `=` equality operator

Allows for __type bindings__

Convention: As types always start with Capital letters; Type bindings also start with capital letters.

Binding types to names will allow to specify recursive types, which is required for many structures in nix.

Binding types to intermediate variables makes it harder to see which declaration is related to the actual code.

e.g.

```nix

/*
 Type: 
   DerivationAttrs = { buildInputs :: [ Derivation ], ... }
   MkDerivationAttrs = DerivationAttrs // { buildInputs :: String }
   mkDerivation :: MkDerivationAttrs -> Derivation
*/
mkDerivation = {pname, version, foo, ...}@args: let
# ...

```

Those bindings should be scoped with an useful mechanism, which could be:

- File wide
- Project wide
- Declaration block
- Same scope as the referenced function binding has

e.g.

```nix
  type:
    foo = Any
```

### prohibited binding

To prevent conflicts and confusion.
__It is strictly prohibited to choose a name your custom type to be the same as:__

- One of the [reserved types](./types.md)

## `?` optional arguments in an AttrSet

e.g.  `{ opt :: ? Int }`

Note: The `type` side contains the `?` operator.

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
