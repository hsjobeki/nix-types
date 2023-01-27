# Lambda

## Abstract definition

A lambda is a function that takes exactly one `argument` and returns exactly one `result`.

It is denoted as follows:

```nix
a -> b
```

Where `a` is the `argument` and `b` is the returned `result`.

Then a typed lambda notation:

```nix
T -> G
```

Denotes that `T` is the type of argument `a` and `G` is the type of result `b`.

> In `lambda` notations arguments do not have names like in e.g. `AttrSet` because they are positional arguments.

## Examples

### Lambda that takes a `String` and returns `String`

`String -> String`

### Function that takes `String` and `Number` and finally returns `String`

`String -> Number -> String`

> As lambdas can take only one argument, the return type of the first lambda expression is a lambda that takes the second argument and returns the final type.

### Function that takes a function

`(String -> String) -> [String] -> [String]`

> Sometimes parenthesis is necessary to clarify order of evaluation
>
> The `()` Parenthesis operator is defined [here](./operators.md)
