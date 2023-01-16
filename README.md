# nix-types RFC (draft)

This Draft of an RFC could be the first step to improve how nix is used as a language.

## :construction: :construction: Any help welcome! :construction: :construction: 

#️⃣ discuss with us on matrix: https://matrix.to/#/#nix-types:matrix.org #️⃣

__Disclaimer: While `types` are great. This doesnt introduce any types into nix.__

My Targets:

- Write down *convention* of the current typing system in nix. (`Type:` doc-stings) 

- Enhance the system so it is possible to type everything in nix using that new system.

- Provide a type-system that could proof correctness of code before runtime.

- Provide a Parser and AST Specification for that type system.

## Scope

- Let the convention be so good, that we can parse the `Type:` into an usefull `AST`.

Currently there is the `type:` pattern which can be parsed from nixdoc. Which is a good start but not enough.
The goal is to build more accurate type comments and have a consistent convention on what is actually allowed and what is not.  

Although nix doesn't have a type system, it has types. (See [here](https://nixos.org/manual/nix/stable/language/values.html))

Because there are different ways for developers to express typings they are very inconsistent.

Generally there are two type systems:

- __Static__
  checked during 'compile' time or development time. 
  So errors can be caught while writing code.
  
  __Does not exist in nix__  
  
- __Dynamic__
  - Fails execution of code based on conditionals.
  - Used in `lib/types.nix`
  - Used in `YANTS`

## Static types

I propose to build a set of simple yet effective `static types` instead of following the dynamic types from the `option types`.

As i am not a type theorist but from my perspective few static types can represent a lot of dynamic ones. 

e.g

| static  	|  option types 	|   	
|---	|---	|	
| String  	|  String 	|
| String  	|  CommaSeperatedString 	|
| String  	|  EmptyString 	|
| String  	|  NonEmptyString 	|

Mainly those are the same `types` from a static perspective because it makes no difference if you have an empty string, or a comma seperated one, you can always perform the same operations on them. like `split` `indexOf` `optionalString` `etc`. Option-Types are only dynamic checks and not real types.

## Abstract

![type-system](./Types.drawio.svg)

Type systems are good:

- A good type-system can proof correctness of code at compile time.

__`doc-strings` are the best possible solution in my opinion. Because they dont alter the nix language itself, but allow for static type checking from external tools. (like `nil`)__

With doc-strings we can give first shot, which might not be 100% perfect and doesn't alter the nix language. Building on top of that we can evaluate the type system and show if it represents the code close enough for a second proposal. Then that second proposal could integrate the types into the nix language itself.

Thats why I decided to give it a try. At least to clearify the conventions of the current type-comment-system.
And introduce a really consistent and reliable `intermediate representation`  of types in nix
In [nipkgs/lib/*](https://github.com/NixOS/nixpkgs/tree/master/lib) there are some files that contain descriptive type comments. And this approach aims to reach high compatibilty with that but also to be intuitive and consistent with the existing language paradigms.

## Convention

The goals:

- __Make all typings (doc-strings) consistent & parseable__.
- all docstrings can be parsed into an `AST`, which can then be used from external tools.
- nixos modules follow the convention and can be used within that system. (either provide the same AST directly or doc-strings)
- some first tools adopt and use the convention. (nil, noogle, documentation, auto-completion in ide's ? , etc.)


### Parser 

Requires some sort of parser, where everything it doesnt accept is an invalid doc-string

> The Parser could then implement and proof the ruleset and vice versa.
> 
> outputs an AST, also described within this Project

## basic rules for writing `type:` comments

1. `type:` starts the type block. Followed by at least one line-break

The type block is never terminated and expands till to the bottom of the `/* multiline comment */`

This is actually how comments are parsed today.

2. Within one block multiple declarations are allowed.
3. All declarations are `PascalCase`, starting with capital letters.
4. Types MUST be choosen from the existing list. (see [below](#List-of-static-Types) )
5. Operators MUST be choosen from the existing list. (see [below](#List-of-Operators) )
6. The Language rules MUST be followed (described in this paper)
7. `AttrSet` and `List` keywords are PROHIBITED. Writers must express explizitly if they want to allow arbitrary values. e.g. `AttrSet` is an alias for `{ ... }` (explained below), same for `List` -> `[ Any ]`
8. AttrSets definitions should include their keys if they dont accept arbitrary values. `{ foo = bar; } # type: { foo :: Any }`
9. Type bindings (explained below) are PROHIBITED to choose names from the reserved list (see [below](#List-of-static-Types) )
10. Spaces between Operators (optional) 

## List of static Types

### Basic

- Bool
- Int
- Float
- String
- Path
- Null

#### The `::` operator


The `::` accepts two arguments:
A `LHS` and `RHS`

Let `U :: T` be a valid usage of the operator.
Then `U` is the `LHS` parameter and `T` is the `RHS` parameter.

The `::` operator takes a property name called `U` and declares the type of `U` to be `T`

It returns an type-expression that can be used as an input to other operators again.

#### The `()` Operator 

Let `(t)` be a semantic group to give precedence to encapsulated term `t`

When the evaluation of a type happens, the term inside `()` gets evaluated first.

### Nested

Nested types MUST always specify their content type.

- AttrSet represented as `{}`
- List represented as `[]`
- Lambda represented as `->`

#### List

__Definition__

Let `[ Any ]` be a list where the elements of that list do not have any type constraints.

Then a List of a specific Type `[ T ]` is a list where all elements fullfill the type constaint `T`.

A list can contain no, one or multiple elements.`

__Examples__

`[ String ]`

`[ Number | Bool ]`

`[ ]`

#### AttrSet

`::`-operator within `AttrSet` 

The `::`-operator maps the Type of its `RHS` over the `Type` on its `LHS`. It can take an `Iterable` or a `single element` on its LHS.

Within Type-declarations for AttrSets it is possible to declare explizit members of an AttrSet like this.

```
  {
    N :: T
  }
```

Then `N` is of type `String` and `N` becomes an __explizit member__ of that AttrSet which references a value of type `T`.
The value of `N` is called the `member name`

Introducing: `[ N :: T ]`-operator, which can only be used within `AttrSet` in `member name` fields.

The `[ N :: T ]`-operator maps over all `member names` of an AttrSet `[N]` and applies the type `T` to each member name `N` if not already done by __explizit member__ declaration (see above).

When there are AttrSets with __dynamic members__ it is possible to declare all those members at once with the `[ N :: T ]` and `::` operator.

Then an AttrSet with list of __dynamic members__ where each member-name `N` references a __value of type__ `V` can be written as.

```
  { 
    [ N :: T ] :: V 
  }
```

__Examples__

```
  # member '.foo' references a value of type string
  # all other members `*` are of type string and each member reference value of type Any.
  { 
    [ name :: String ] :: Any, 
    foo :: String 
  }
```

```
  { foo :: Any } 
```

```
   {} 
```

where the member names `[ N :: T ] are an empty list.

__useful `${}` Shortcut__

`${N} = [ N :: String ]`

If we take into account that in AttrSets `names` (`N`) are always of type `String` the user can omit the `String` Keyword completely, and instead give only the names. `N`

That rule allows for intuitive usage of names within type definitions of an AttrSet

```
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

#### Lambda

__Definition__

let Lambda `= Any -> Any` the set of all possible lambdas.
Let the `LHS` of `->` be the `Argument type` `T` of a `lambda` and the `RHS` the return value type `U`

Then a lambda that takes type `T` and returns type `U` can be declared as the subset of all possible lambdas with both `T` on the `LHS` and type `U` on the RHS.

__Examples__

```
Number -> Number

[ String ] -> ( String -> Number ) -> [ Number ]

(Path -> String) -> { ${name} :: Path } -> { ${name} :: String }
```

### `|` syntactic `Or` can be used for composition or enums

Let `T` and `U` be different Types.
Then the `|` operator evaluates to either `T` or `U`.

__Examples__

`Float | Int`

`( Number | Bool ) | Int`

`{ opt :: Int | String }`

### Composed Types`

Now with the basics clearified we can finally define composed types.

- Number `= Int | Float`

A `Number` can be either an `Int` or a `Float` type.

- Any `= [ Any ] | { [ name :: String ] :: Any } | (Any -> Any) | Bool | Int | Float | String | Path | Null` 

An `Any` can be either a basic type or a nested type of `Any`

__Global Types__

Some types are commonly used within nix and nixpkgs therefore it makes sense to have some more Types that are always availabe.

Those are types defined globally within nix as they are almost always needed.

- StorePath `::= Path`

- Derivation `::= { # TODO... }`

- Package `::= # TODO..`


## List of Operators

__All Operators SHOULD be used with surrounding whitespaces.__

### `::`  declares the type.

The variable name on the LHS is declared to have the `type` on the RHS

e.g. `name :: Any`

### `()` Parenthesis 

Paranthesis to clearify order of type evaluation

e.g. `( a -> b ) | Bool`

### `,` Seperator for subseqeuent entries (like in AttrSet)

e.g. `{ foo :: Any, bar :: Any }`

### `//` syntactically `merges` Types of AttrSets

`{ foo :: String } // { bar :: Any }` => `{ foo :: String, bar :: Any }`

`{ foo :: String } // { ${names} :: Any }` => `{ foo :: String, ${names} :: Any  }`

Overwrites occur like in the nix language itself

`{ foo :: String } // { foo :: Any }` => `{ foo :: Any }`

### `=` equality operator. Allows for __type bindings__

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

Those bindings should be scoped with an usefull mechanism, which could be:

- File wide
- Project wide
- Declaration block
- Same scope as refenced function binding has


e.g. 
```
  type:
    foo = Any
```

### `?` optional arguments in an AttrSet.

e.g.  `{ opt :: ? Int }`

Note: The `type` side contains the `?` operator.

### `"` Literal type

Literals are strings, of specififc values.

e.g. `{ foo :: "bar" }` specifies the name `foo` to be only of value "bar"

```
{
  foo = "bar";
}
```

This can be usefull for constant fields, which are always the same across specififc types.

### `...` - arbitrary input values

can only be used within an AttrSet

`...` = `[ String ] :: Any` within an AttrSet context

e.g. 

```
/*
  Type: foo :: { bar :: Any, ...} -> Any
*/
Foo = {bar, ...}@inp:
#...
```

## Some Best practices

- Use the `Type` postfix for creating typing aliases
- Start with the most abstracted declaration first. 
- Linebreaks and trailing commas in AttrSets and Lists.
- Create aliases to reduce complexity if needed.
- Use type comments wherever possible to increase maintainabilty and discoverabilty for others 

e.g.

```

/*
  Type:
    packageSetInfo = {...} -> InfoType
    InfoType = {foo :: Any, ...}
*/
packageSetInfo = attrs: getInfo { inherit attrs; };

```

## About the modules (nixos modules)

nixos modules typing system is dynamically evaluated. It misses (like everyting else) __static__ analysis possibilities.

With the power of both worlds, static & dynamic, nix developers should be able to get high quality code up and running more reliable, faster and with less brainload. So developers can focus on more important parts of their nix applications.

The proposed change for `option types` provide an `AST` attribute that implements the `AST` described in this project.

__Example_

```nix

listOf = elemType: mkOptionType rec {
    name = "listOf";
    description = "list of ${optionDescriptionPhrase (class: class == "noun" || class == "composite") elemType}";
    # new ------------------
    documentation = {
      outputType = {
        type = "List";
        # ... See AST spec.
      };
    };
  
    # ------------------

    descriptionClass = "composite";
    check = isList;
    merge = loc: defs:
    #...
      
```

> The in don't care about exact name, its is more important to have access to the AST than what it is called.

### Potential Impact

Writing type comments is very tideous and those comments can drift over time, and at one point they might describe not exctly what is going on.
So enhancing nixos modules and improving documentation system on that `self-documenting` system seems really beneficial to me. 

__So the same tools can process nixos modules, without great additions, or the need for a second parser__

### Connect dynamic and static typings

As the __dynamic types__ and __static types__ are very different the need to be connected somehow.

I propose the following:

#### Mapping of Dynamic types into Static types.

As the dynamic types already exist, that initial mapping should be done. In the dynamic world the same type may have a different name.

e.g. `Derivation` vs `Package`

__every option type from lib/types.nix must be mapped to a declaration from this paper.__
