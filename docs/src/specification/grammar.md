# Tokens

## The following tokens do exist

|Token | Name | Purpose |
|---|---|---|
| {} | Left / Right Braces | Set Types; Interpolation together with the $-Token; |
| [] | Left / Right Brackets | List Types |
| () | Left / Right Parens | Structure Types,override precedence |
| @ | At | Binding Typed values to names |
| :: | Double Colon | Declares the type (RHS) of name (LHS) |
| . | Dot | Used to create explicit float values; Usage for accessing child properties in sets is not yet implemented. Some usage example: `0.1` `.5e-9` `5.e2` (This is just native nix) |
| ? | Question | Optional properties of sets |
| ; | Semicolon | Terminate set entries |
| -> | Implication | Define Lambdas e.g. Int -> String |
| // | Update | Update Attribute Set Types. Can be used to partially update and insert name-value-types |
| - | Sub | Used for explicit negative numbers (-2.0), subtraction is NOT supported (e.g 3 - 1) |
| <> | Path | Define valid paths as explicit values |
| \| | Pipe | Create Type-Unions e.g. `Int \| Float` |
| ${} | Interpolation | Used for naming / referring to dynamic names |
| " | String body (single-line) | explicit string values |
| '' | String body (multi-line) | explicit string values |
| `[0-9]*` | Integer | explicit integer values |

## The following keywords do exist

Only the `let .. in` keyword are supported.

|Keyword |  Purpose |
|---|---|
| let | declaration block |  
| in | what is yielded |

## Keywords removed

As complexity is already quite high. It is considered best to not support the following keywords in the type language.

|Keyword | |
|---|---|
|assert |  |
|if |  |
|else |  |
|then |  |
|inherit |  |
|or |  |
|rec |  |
|with |  |

### Reserved Types

All types are written Uppercase by convention to visually distinguish them from variables and actual values.

Some composed types are handled as native types as well. They are well defined internally, but can be used natively in the type language.

|Keyword |
|---|
| Bool |
| Int |
| Float |
| String |
| Path |
| Null |
| Number |
| Derivation |

### Tokens removed

|Token | Name | Purpose|
|---|---|---|
| , | Comma | Originally used for separation in lambda-patterns, replaced by ; (Semicolon) |
| : | Colon | Originally used for lambda, patterns; in Type context replaced by -> |
| = | Assign | Does not exists, use :: (Double Colon) to declare |
| ++ | Concat | Does not exists, use \| (Pipe) to create list unions |
| / | Div | Not used |
| /**/ | Multiline Comment | Not used |
| + | Add | Not used |
| * | Mul | Not used |
| && | And | not used |
| \|\| | Or | not used, use single `\|` (Pipe) |
| < <=, =>, >  | Less, Less or equal, More or equal, More | comparison not used |

## _Internal context

|Context | Purpose |
|---|---|
| MultilineCommentBody | Initial Context, every character is just the content of the comment. |
| TypeBlock | Started by `{whitespace}Type:`; Every following character is tokenized with the type grammar. |
| ExampleBlock | Started by `{whitespace}Example:`; Everything followed is just a comment string content. Token may not be needed? |
| StringBody; StringEnd; Interpol; InterpolStart; Path | Regular nix tokenization context |
