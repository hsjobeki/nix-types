# AttrSet

## Abstract

An `AttrSet` type can be though of as just a `pair` of ...

## Redefining some operators

`::`-operator within `AttrSet` contexts

The `::`-operator maps the Type of its `RHS` over the `Type` on its `LHS`. It can take an `Iterable` or a `single element` on its LHS.

Within Type-declarations for AttrSets it is possible to declare explicit members of an AttrSet like this.

```nix
  {
    N :: T
  }
```

Then `N` is of type `String` and `N` becomes an __explicit member__ of that AttrSet which references a value of type `T`.
The value of `N` is called the `member name`

Introducing: `[ N :: T ]`-operator, which can only be used within `AttrSet` in `member name` fields.

The `[ N :: T ]`-operator maps over all `member names` of an AttrSet `[N]` and applies the type `T` to each member name `N` if not already done by __explicit member__ declaration (see above).

When there are AttrSets with __dynamic members__ it is possible to declare all those members at once with the `[ N :: T ]` and `::` operator.

Then an AttrSet with list of __dynamic members__ where each member-name `N` references a __value of type__ `V` can be written as.

```nix
  { 
    [ N :: T ] :: V 
  }
```

> __Examples__

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

__useful `${}` Shortcut__

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
