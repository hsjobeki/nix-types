# Formals

The following section specifies how to write `type hints` as comments

> `type:` Type

Every specified formal rule has the following format:

> [NT000] - Rule description

## The rules

- [NT000] - The Language rules MUST be followed (described in this project)
- [NT001] - `Type hints` can only occur within multiline comments `/* */`
- [NT002] - `Type:` starts the type block. Followed by at least one line-break (case sensitive)
- [NT003] - Within one block multiple declarations are allowed.
- [NT004] - All notations are `PascalCase`; starting with capital letters.
- [NT005] - Types MUST be chosen from the existing list. see [primitives](./basic.md)
- [NT006] - Operators MUST be chosen from the existing list. see [operators](./operators.md)
- [NT007] - explicit `AttrSet` and `List` keywords are PROHIBITED. See the correct usage [here](./complex.md)
- [NT007] - explicit `Any` is PROHIBITED. Correct usage with type variables see [here](./type-variables.md)
- [NT008] - [complex types](./complex.md) MUST include their members explicitly.
- [NT009] - Allowing arbitrary values within [complex-types](./complex.md) is possible but requires explicit statements.
- [NT009] - `Type variables` must be written in LOWERCASE to prevent confusion.
- [NT010] - When using [Type bindings](./operators.md#type-binding) it is PROHIBITED to choose names from the [reserved list](./types.md)
- [NT011] - Spaces between Operators (recommended)
- ... more concrete rules may follow when implementation has started

> Hint: The type block is never terminated and expands till to the bottom of the `/* multiline comment */`. This is actually very close to how comments are parsed today.

### Example of a valid and nicely formatted `type hint`

> Usage of operators will be explained in the following chapters

```nix
  /* Return an attribute from nested attribute sets.
    ...
    Free Text goes here 
    ...
    
    Type:
        attrByPath :: [ String ] -> a -> { ${path} :: b } -> ( a | b )
  */
  attrByPath = ...
```

This function takes a path as list of strings and then traverses the attrSet until it may find a value of type 'b' otherwise it returns the default value of type 'a'.
