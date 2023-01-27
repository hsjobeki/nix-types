# List

## Abstract definition

Let `[ a ]` be a list where the elements of that list do not have any type constraints.

Then a List of a specific Type `[ T ]` is a list where all elements `a` fullfil the type constraint `T`

> A list `[]` can contain **no**, **one** or **multiple** elements. (0...n)

## Examples

### list of string

`[ String ]`

### list of string or bool

`[ Number | Bool ]`

-> see the `|` {or} operator defined [here](./operators.md)

### empty list

`[ ]`

### list of any attrSet

`[ { ... } ]`
