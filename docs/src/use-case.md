# Use-cases

## Standardized (parsable) format to communicate type

There are simple functions with only few and basic arguments.

```nix
Number -> Number -> Number
```

But there are also complex functions, that other functions or nested attrsets as arguments.
Communicate the rules those structures may have to follow is crucial.

```nix
String -> [ { ${name} :: a } ] -> [a]
```

We think; Understanding those `type` signatures with automated tooling is significant.
For that to work we need a syntax that follows strict rules.

## Provide hover and autocomplete information's on expressions

As a developer i'd like to get suggestions like:

- How many arguments does a certain function take?
- Type of the arguments?
- In case of Attribute-set arguments:
  - Which attributes of the required attributes are still missing?
  - Which optional attributes can i specify additionally?
- In general describe the type of an expression

## Provide static type checks at development time

A static tool could check the actual type of the documented expression against the specified type.
It is unclear how this would work, but it could certainly be done.

We think an approach would also include pre-evaluation of certain points, that are very hard to analyze statically.
