# Introduction

This Draft of an RFC could be the first step to improve how nix is used as a language.

Generally there are two type systems:

- __Static__
  checked during 'compile' time or development time.
  So errors can be caught while writing code.
  
  __Does not exist in nix__  
  
- __Dynamic__
    - Fails execution of code based on conditionals.
    - Used in `lib/types.nix`
    - Used in `YANTS`

## Proposal: Type hints

I propose to build a set of simple yet effective `type hints`, which would mimic `static types` instead of investing to much into the dynamic types approach,
or trying to bring static types into the dynamic nix language.

By using `type hints` many additional possibilities even for dynamically typed languages arise.

- determine whether a program would actually work or not, just by looking at the types.
- provide autocompletion on typed variable- and function-bindings.
- automatically generate good documentation directly from code.
- ...

## Dynamic type validations are still needed

As type hints cannot provide any runtime assurances it is still needed to place guards in critical places.
But instead of trying to represent a non-existent static type system the dynamic approach should transparently communicate what it really does: "Runtime validation".
