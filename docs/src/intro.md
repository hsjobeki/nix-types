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

## Why static types

I propose to build a set of simple yet effective `static types` instead of following the dynamic types approach.

By using static types many additional possibilities even for dynamically typed languages arise.

- determine whether a program would actually work or not, just by looking at the types.
- provide autocompletion on typed variable- and function-bindings.
- automatically generate good documentation directly from code.
- ...
