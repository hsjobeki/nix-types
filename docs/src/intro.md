# Introduction

🚧🚧🚧 RFC Under construction 🚧🚧🚧🚧

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
But instead of trying to represent a non-existent static type system;
The dynamic approach should transparently communicate what it really does: "Runtime validation".

## Analogy to other dynamic languages

![other-dynamic-languages](./assets/Other-dynamic-languages.drawio.png)

## Vision

This project can be seen as a first step of possibly many.

There are many opinions about whether static types make sense at all in nix.

But this project does not discuss those. Instead i propose that "types" in whatever form do make total sense. And will give much value to dynamic languages.
Which can undoubtedly be seen on the above examples (Python, Javascript).

__Discussion about__
```Does typing make sense in a dynamic Language```
__is open, but history has a clear answer to that.__

By specifying the comments and not directly integrating types
into the nix language this project stays downwards compatible
and allows an intermediate phase of transition and validation.

In that validation phase the system can obtain feedback and corrections.

It could then be made so good, that it really represents nix's type behavior.

> NOTE: Currently out of scope of this project.
> Change the nix language to natively support type annotations. (Even if they are not real static types)
> `Types` could then be built from the findings of this project.
