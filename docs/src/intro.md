# Introduction

This Draft of an RFC could be the first step to improve how nix is used as a language.

🚧 🚧 🚧 Currently - Under construction 🚧 🚧 🚧

#️⃣ discuss with us on matrix:[https://matrix.to/#/#nix-types:matrix.org](https://matrix.to/#/#nix-types:matrix.org) #️⃣

Generally there are two type systems:

- __Static__
  checked during 'compile' time or development time.
  So errors can be caught while writing code.
  
  __Does not exist in nix__  
  
- __Dynamic__
    - Fails execution of code based on conditionals.
    - Used in `lib/types.nix`
    - Used in `YANTS`

## Why static types

I propose to build a set of simple yet effective `static types` instead of following the dynamic types from the `option types`.

As i am not a type theorist but from my perspective few static types can represent a lot of dynamic ones.

e.g

| static   |  option types  |
|--- |--- |
| String   |  String  |
| String   |  CommaSeparatedString  |
| String   |  EmptyString  |
| String   |  NonEmptyString  |

Mainly those are the same `types` from a static perspective because it makes no difference if you have an empty string,
or a comma separated one, you can always perform the same operations on them.
like `split` `indexOf` `optionalString` `etc`. Option-Types are only dynamic checks and not real types.
