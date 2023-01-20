# AttrSet

## Abstract

An `AttrSet` type can be thought of as just a list of `pair of name and value`.

- `fst` on `pair of name and value` yields the `name`.
- `snd` on `pair of name and value` yields the `value`.

Then `AttrSet` can be written as:

```nix
[Pair(name,value)]
```

when you reference a specific **name** on an AttrSet; You basically implicitly apply a `filter` or `find` operation on that list where the `fst` of the `mapped entry` equals the referenced **name**

For simplicity this is called a `member name` from now on, if that name exists.

If that name does not exist it is called `non-existing member name`.

## Redefining some operators

`::`-operator within `AttrSet` contexts

The `::`-operator maps the Type of its `RHS` over the `Type` on its `LHS`. It can take an `Iterable` or a `single element` on its LHS.

Within Type-declarations for AttrSets it is possible to declare an explicit `member name` of an AttrSet like this.

```nix
  {
    N :: T
  }
```

Then `N` is of type `String` and `N` becomes a `member name` of that AttrSet. The `snd` operation on the entry of `N` would yield a value of type `T`.

Introducing: `[ N :: T ]`-operator, which can only be used within `AttrSet` in `member name` fields.

The `[ N :: T ]`-operator maps over all `member names` of an AttrSet `[N]` and applies the type `T` to each member name `N` if not already done by explicit member__ declaration (see above).

When there are AttrSets with dynamic members it is possible to declare all those members at once with the `[ N :: T ]` and `::` operator.

Then an AttrSet with list of dynamic members where each member-name `N` references a **value of type** `V` can be written as.

```nix
  { 
    [ N :: T ] :: V 
  }
```

### Examples

```nix
  # member '.foo' references a value of type string
  # all other members `*` are of type string and each member reference value of any variable Type.
  { 
    [ name :: String ] :: a, 
    foo :: String 
  }
```

```nix
  { foo :: a } 
```

```nix
   {} 
```

where the member names `[ N :: T ] are an empty list.

## useful `${}` Shortcut

`${N} = [ N :: String ]`

If we take into account that in AttrSets `names` (`N`) are always of type `String` the user can omit the `String` Keyword completely, and instead give only the names. `N`

That rule allows for intuitive usage of names within type definitions of an AttrSet

```nix
/*
type:
  packageMap :: { 
    ${pname} :: {
      ${version} :: Derivation
    }
  }
*/
packageMap = {
  "gcc-utils" = {
    "1.2.3" = builtins.Derivation {...};
    };
  # ...
  };
```
