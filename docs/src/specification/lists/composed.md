<!-- markdownlint-disable MD041 -->
<!-- markdownlint-disable MD013 -->
### Composed

---

| Type  | Composition | Description  |
|---|---|---|
| `Number` | `Int {or} Float` | The `Number` is either of type `Int` or of type `Float` |
| `Any` | `?` | There is no `Any` type and it is explicitly prohibited to use the Any type. Use type variables instead if you want to allow variable type signatures. |
| `StorePath` | `Path` | The `StorePath` is just a meaningful alias of the type `Path` |
| `Derivation` | `{ ... }` | TODO: `Derivation` is just a special AttrSet. |
| `Package` | `Derivation {or} {...}` | TODO: `Package` is either a Derivation or a special AttrSet with `name` xy in it.  |

> used operators are defined in the [operators chapter](operators.md)
