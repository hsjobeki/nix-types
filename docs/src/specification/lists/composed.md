<!-- markdownlint-disable MD041 -->
<!-- markdownlint-disable MD013 -->
### Composed

---

| Type  | Composition | Description  |
|---|---|---|
| `Number` | `Int {or} Float` | The `Number` is either of type `Int` or of type `Float` |
| `Any` | `[ Any ] {or} { [ name :: String ] :: Any } {or} (Any -> Any) {or} Bool {or} Int {or} Float {or} String {or} Path {or} Null` | Any is either one of the primitives or a complex type or any other composable type |
| `StorePath` | `Path` | The `StorePath` is just a meaningful alias of the type `Path` |
| `Derivation` | `{ ... }` | TODO: `Derivation` is just a special AttrSet. |
| `Package` | `Derivation {or} {...}` | TODO: `Package` is either a Derivation or a special AttrSet with `name` xy in it.  |

> used operators are defined in the [operators chapter](../operators.md)
