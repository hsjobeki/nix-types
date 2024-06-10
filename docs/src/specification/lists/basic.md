<!-- markdownlint-disable MD041 -->
### Primitives

---

| Type  | Description  |
|---|---|
| Bool | A value of type boolean can be either `true` or `false`  |
| Int | TODO: The classical integer type with 32-bit or 64-bit depending on the current system? |
| Float | A float with 64 bits? |
| String | A string of any unicode characters  |
| Path | Path referencing a location or a file. With no assumptions on the existence of that |
| Null | The value null |

## The `Any` type (upper boundary)

The `Any` type is often used to explicitly allow arbitrary values. However,
the Any type is complex and doesn't add much value to a type system. Instead, we should use type variables whenever possible.

Interestingly there are two different `Any` types:

e.g. If we look at CUE-lang (which is also inspired by nix)

> CUE defines the values bottom, or error, (denoted _|_) that is an instance of all types and top, or any, (denoted _) of which all types are an instance.

- `TOP any` all types are an instance of that. You can imagine it as the TOP-most set, that includes every type. But no value has that type.

- `Bottom any` which is an instance of all types. This is kind of the imaginary value that has
the any type. Still, doesn't contain any value. Which could also be denoted: `Never` or `Empty Type` it is a type that is the subtype of any type.

The following is a nice quote from the Typescript world

> The any type is so dangerous because it exists outside of the type tree.
> It is both a top and bottom type.
> Everything can be assigned to it and it can be assigned to everything else. ...

I think it might make most sense to define `Any` as `TOP` in our type system.
This means it is the upper boundary of our type system. All types within the system are a subtype of `Any`.

## `Never` (lower boundary)

I think it makes most sense to have a distinct `Bottom` type.
Other type systems call this: `Bottom any` or `Empty Type`. This is the lower boundary of our type system. All types are a supertype of `Never` and that is true for all types that may eventualy exist in this type system.

The easiest way of thinking about `Never` is this example:

```nix
let
 # a -> Never
 f = x: abort "now";
 # Never <- f 42
 result = f 42;
in
 # result :: Never
 result
```

A function `f` that takes `Any` argument and since it aborts the evaluation it returns `Never`.

`Never` might need its own chapter since it requires understanding lazyness in the language to determine which expression returns never during evaluation.

## The Bool type

As the Bool type can only have two values (true/false)

The following is the definition of the Bool type

```hs
Bool :: true | false
```
