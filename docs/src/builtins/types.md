# Builtin Types

This section contains the proof that all nix `builtins` can be meaningfully and correctly annotated with the proposed type system.

There are currently **89** builtin functions, that can be discovered with

> run `nix __dump-builtins`

Every builtins signature can be found below (sorted alphabetically)

## Builtins - TODO: The type signatures are not compliant yet

### abort

```nix
 String
```

### add

```nix
 Number -> Number -> Number
```

### all

```nix
 (a -> Bool) -> [a] -> Bool
```

### any

```nix
 (a -> Bool) -> [a] -> Bool
```

### attrNames

```nix
 {AttrSet} -> [a]
```

### attrValues

```nix
 AttrSet -> [a]
```

### baseNameOf

```nix
 String -> String
```

### bitAnd

```nix
 Int -> Int -> Int
```

### bitOr

```nix
 Int -> Int -> Int
```

### bitXor

```nix
 Int -> Int -> Int
```

### break

```nix
 a -> a
```

### catAttrs

```nix
 String -> [ { ${name} :: a } ] -> [a]
```

### ceil

```nix
 Float -> Int
```

### compareVersions

```nix
 String -> String -> Int
```

### concatLists

```nix
 [List] -> []
```

### concatMap

```nix
 (a -> b) -> [a] -> [b]
```

### concatStringsSep

```nix
 String -> [String] -> String
```

### deepSeq

```nix
 a -> b -> b
```

### dirOf

```nix
 String -> String
```

### div

```nix
 Number -> Number -> Number
```

### elem

```nix
 a -> [b] -> Bool
```

### elemAt

```nix
 [a] -> Int -> b
```

### fetchClosure

```nix
 AttrSet -> AttrSet
```

### fetchGit

```nix
 AttrSet -> AttrSet
```

### fetchTarball

```nix
 AttrSet -> AttrSet
```

### fetchurl

```nix
 String -> AttrSet
```

### filter

```nix
 (a -> Bool) -> [a] -> [b]
```

### filterSource

```nix
 (Path -> String -> Bool) -> Path -> StorePath
```

### floor

```nix
 Float -> Int
```

### foldl'

```nix
 (a -> b -> c) -> a -> [b] -> c
```

### fromJSON

```nix
 String -> a
```

### functionArgs

```nix
 (a) -> AttrSet
```

### genList

```nix
 (a -> b) -> a -> [b]
```

### genericClosure

```nix
 AttrSet -> [AttrSet]
```

### getAttr

```nix
 String -> AttrSet -> a
```

### getEnv

```nix
 String -> String
```

### getFlake

```nix
 AttrSet -> AttrSet
```

### groupBy

```nix
 (a -> b) -> [a] -> AttrSet
```

### hasAttr

```nix
 String -> AttrSet -> Bool
```

### hashFile

```nix
 String -> Path -> String
```

### hashString

```nix
 String -> String -> String
```

### head

```nix
 [a] -> a
```

### import

```nix
 Path -> a
```

### intersectAttrs

```nix
 AttrSet -> AttrSet -> AttrSet
```

### isAttrs

```nix
 a -> Bool
```

### isBool

```nix
 a -> Bool
```

### isFloat

```nix
 a -> Bool
```

### isFunction

```nix
 a -> Bool
```

### isInt

```nix
 a -> Bool
```

### isList

```nix
 a -> Bool
```

### isNull

```nix
 a -> Bool
```

### isPath

```nix
 a -> Bool
```

### isString

```nix
 a -> Bool
```

### length

```nix
 [a] -> Int
```

### lessThan

```nix
 Number -> Number -> Bool
```

### listToAttrs

```nix
 [{name :: String; value :: a}] -> AttrSet
```

### map

```nix
 (a -> b) -> [a] -> [b]
```

### mapAttrs

```nix
 (a -> b -> c) -> AttrSet -> AttrSet
```

### match

```nix
 String -> String -> Bool
```

### mul

```nix
 Number -> Number -> Number
```

### parseDrvName

```nix
 String -> AttrSet
```

### partition

```nix
 (a -> Bool) -> [a] -> AttrSet
```

### Path

```nix
 AttrSet -> StorePath
```

### pathExists

```nix
 Path -> Bool
```

### placeholder

```nix
 String -> String
```

### readDir

```nix
 Path -> AttrSet
```

### readFile

```nix
 Path -> String
```

### removeAttrs

```nix
 AttrSet -> [a] -> AttrSet
```

### replaceStrings

```nix
 [String] -> [String] -> String -> String
```

### seq

```nix
 a -> b -> b
```

### sort

```nix
 (a -> b -> Bool) -> [a] -> [b]
```

### split

```nix
 String -> String -> [String]
```

### splitVersion

```nix
 String -> [String]
```

### StorePath

```nix
 StorePath -> StorePath
```

### stringLength

```nix
 String -> Int
```

### sub

```nix
 Number -> Number -> Number
```

### substring

```nix
 Int -> Int -> String -> String
```

### tail

```nix
 [a] -> a
```

### throw

```nix
 String
```

### toFile

```nix
 Path -> String -> StorePath 
```

### toJSON

```nix
 a -> String
```

### toPath

```nix
 String -> Path
```

### toString

```nix
 a -> String
```

### toXML

```nix
 a -> String
```

### trace

```nix
 a -> b -> b
```

### traceVerbose

```nix
 a -> b -> b
```

### tryEval

```nix
 a
```

### typeOf

```nix
 a -> String
```

### zipAttrsWith

```nix
 (String -> [a] ) -> [a] -> AttrSet
```
