# Examples

This section contains some illustrative examples that show how to practically apply the specification in concrete use-case-scenarios.

## Short recap of the basic syntax

```nix
# lib.lists.any

/* 
    Type: 
        any :: ( Any -> Bool ) -> [ Any ] -> Bool
         ↑   ↑        ↑        ↑             ↑
         |   |        |        |             └── final return value is a Bool
         |   |        |        └── The first function returns a function that takes a list of Any      
         |   |        └── function that takes Any and returns True or False (Boolean)
         |   └── declares the type as follows
         └── referenced binding name
*/
any = builtins.any or (pred: foldr (x: y: if pred x then true else y) false);
```

## first examples

```nix
# lib.lists.remove

/*
    Type:
        remove :: Any -> [ Any ] -> [ Any ]
*/
remove = #...
```

```nix
# lib.attrsets.hasAttrByPath

/*
Type:
    hasAttrByPath :: [ String ] -> { ... } -> Bool
*/
hasAttrByPath = # ...
```

```nix
/*
# example file from the dream2nix framework

type:
    depsTree :: DependencyTree

    DependencyTree = { ${name} :: { ${version} :: DependencyAttrs } }
    DependencyAttrs = { { deps :: DependencyTree, derivation :: Derivation } }
*/
depsTree = let 
# ...
```

```nix
/*
Type:
    getAttrs :: [ String ] -> { ... } -> { ... }
*/
getAttrs = names: attrs: genAttrs names (name: attrs.${name});
```

## Open points

With `Any` there is no indication if all arguments must contain the same single type `a` thus it would be more precise with **type variable** instead of the explicit `Any` type.

```nix
# lib.lists.remove

/*
    Type:
        remove :: a -> [ a ] -> [ a ]
*/
remove = #...
```

The following is a hard one. We are not sure yet how to present that type `b` is present somewhere in a nested set.

It gets returned, if the referenced value is present in the attrSet; default value of type `a` otherwise.

`a` can be equal to `b` but generally speaking it doesn't have to.

```nix
# lib.attrsets.attrByPath

/* Return an attribute from nested attribute sets.

    Type:
    attrByPath :: [ String ] -> a -> { ... } -> b | a
*/
attrByPath = # ...
```

We are not able yet to dynamically create members of an AttrSet from a given type signature.

```nix
/*
Type:
    getAttrs :: [ String ] -> { ... } -> { ... }
                    ↑                      ↑
                    └──────────────────────┴─ The list of String matches the member name of the returned AttrSet

*/
getAttrs = names: attrs: genAttrs names (name: attrs.${name});
```

Thus it might make sense to generally allow context on ListItems as follows. (Optional Context)

```nix
    getAttrs :: [ name :: String ] -> { ... } -> { [ name :: String ] :: Any }
                    ↑                                 ↑
                    └─────────────────────────────────┴─ The list of String matches the member-names of the returned AttrSet
```
