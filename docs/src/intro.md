# Introduction

This is my (draft) writing to try to formalize type convention and formals for the nix language.

To support the convention utilize doc-comments.

Example:

````nix
{
  /**
    # Type

    ```
    mapAttrs :: ( String -> a -> b ) -> { ${name} :: a; } -> { ${name} :: b; }
    ```
  */
  mapAttrs = ...
}
````

## Motivation

By specifying the syntax of the type section within nix this opens a wide range of possibilities.

- consistent function / API documentation. (I.e. used by noogle.dev)
- Type checking - whether you call a function with the expected types.
- Language Server (LSP) support - display inline or hover type signatures.
- Encourage Modular architectures, with well-defined and documented interfaces.
- Static type / correctness checking

## Detailed implementation

I propose to formulate a strict type syntax. Be it used for doc-comments or maybe even integrated into the language itself.

### Syntax follows nix

The syntax is mainly inspired and as close as possible to the Nix language. Everyone who knows Nix can easily learn the type-language for it.
There are only very few modifications that are self-explaining and intuitive. While we could imagine a way more complex language, that allows expressing every last detailed relationship.
I think it is more important to provide a good abstraction and find the right compromise between simplicity and expressiveness.
