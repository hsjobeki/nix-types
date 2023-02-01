# Operations

The official documentation about the operators in nix can be found [here](https://nixos.org/manual/nix/stable/language/operators.html)

🚧🚧🚧 This site is WIP

## Official List

| Name                                   | Syntax                                     | Associativity | Precedence |
|----------------------------------------|--------------------------------------------|---------------|------------|
| Attribute selection                    | *AttrSet* `.` *AttrPath* \[ `or` *expr* \] | none          | 1          |
| Function application                   | *func* *expr*                              | left          | 2          |
| Arithmetic negation                    | `-` *number*                               | none          | 3          |
| Has attribute                          | *AttrSet* `?` *AttrPath*                   | none          | 4          |
| ist concatenation                      | *list* `++` *list*                         | right         | 5          |
| Multiplication                         | *number* `*` *number*                      | left          | 6          |
| Division                               | *number* `/` *number*                      | left          | 6          |
| Subtraction                            | *number* `-` *number*                      | left          | 7          |
| Addition                               | *number* `+` *number*                      | left          | 7          |
| String concatenation                   | *string* `+` *string*                      | left          | 7          |
| Path concatenation                     | *path* `+` *path*                          | left          | 7          |
| Path and string concatenation          | *path* `+` *string*                        | left          | 7          |
| String and path concatenation          | *string* `+` *path*                        | left          | 7          |
| Logical negation (`NOT`)               | `!` *bool*                                 | none          | 8          |
| Update                                 | *AttrSet* `//` *AttrSet*                   | right         | 9          |
| Less than                              | *expr* `<` *expr*                          | none          | 10         |
| Less than or equal to                  | *expr* `<=` *expr*                         | none          | 10         |
| Greater than                           | *expr* `>` *expr*                          | none          | 10         |
| Greater than or equal to               | *expr* `>=` *expr*                         | none          | 10         |
| Equality                               | *expr* `==` *expr*                         | none          | 11         |
| Inequality                             | *expr* `!=` *expr*                         | none          | 11         |
| Logical conjunction (`AND`)            | *bool* `&&` *bool*                         | left          | 12         |
| Logical disjunction (`OR`)             | *bool* `\|\|` *bool*                       | left          | 13         |
| Logical implication                    | *bool* `->` *bool*                         | none          | 14         |

## Type signatures

From a type perspective those operators can be seen as lambda functions. Every operator takes arguments of one type and returns a type.
The following list was created to clarify the type signatures.

Some operators take one or two arguments, they can either take them from left or right hand side.

### Some formals

for simplicity

- `R` is used as `Right hand side` type.
- `L` is used as `Left hand side` type.
- When using the proposed lambda notation, the first argument is always `LHS` and the second the `RHS` of the discussed operator.
- `T,U,W` are used as generic Type variables mostly do denote the resulting type, or types of free variables.
- `a`,`b`,... can be of any type. They are type variables.
- `x` is a functions input value
- `y` is a functions return value

### '.' - Attribute selection

Usage of the (`.`) operator:

`L` must be an AttrSet, and `R` must evaluate to type `String`

If R itself contains an expression this is parenthesized with the `${ }` operator.
This operator tells the parser to evaluate the expression in `R` before passing it to the (`.`) function.

```nix
 L(.)R = L.R 
```

```nix
 L(.)R = L -> R -> T
 L(.)R :: { ${name} :: a } -> String -> a
```

### Function application

Let `f` be a function; `x` its applied argument and `y` the applied function result.

```nix
 y = f x
```

For comprehensiveness let the type signature of the above function be `L` and the applied argument `R`.

blow the argument `R` is applied to a function of type `L`.

```nix
 L R = L -> R -> T
 L R :: (a -> b) -> a -> b 
```

### '-' - minus operator

```nix
 y = x - 1
```

> (`-`) is a special case where the `RHS` is applied first to the function.
> So if the `LHS` was omitted it gets the value `0` as type `Int`.

```nix
 L(-)R :: R -> L -> T
 (-) :: ( Int | Float ) -> ( Int | Float | 0 ) -> (Int | Float)
```

The minus operator is a dependent type
It depends on the received argument value

- If it receives two `Int` types it returns `Int`
- but if one or both of its arguments is of type `Float` it returns type `Float`
- The type signature is correct using the `Number` type, but it lacks the information about its `value dependency`.

### '+' - plus operator

### '?' - Has attribute

### '++' - List concatenation

### '<' '>' - comparison

### '==' - Equality

### '!=' - Inequality

### '*' - Multiplication

### '!' - Negation

### '//' - Update

### '&&' '||' '->' - Logical
