# General Blocks

Currently there are two main blocks available

- {Description}
- Example
- Type

I think we should keep the current general structure but clarify the rules for usage a bit.

## General Rules

[N000] - This ruleset only applies to multiline comments. Thus it makes it much easier to use and to parse later on.
[N001] - Every block start with the appropriate (case-sensitive) `keyword` followed by a colon (:). e.g. `Type:` (see full list of `keywords`)
[N002] - There can only be one "Type:" statement, but multiple "Example:" statements.
[N002] - The `keyword` must be the first word in the `line`. e.g. the following token sequence starts a block: `[\n, {Whitespace(N)} {Keyword} {Colon} ]`.
[N002] - Every block is continued until the next block starts. To the very end of the comment otherwise.
[N003] - `Type:` blocks are validated using the custom parser provided by this project.
[N003] - Any block can be inspected by using the custom parser provided by this project.

## List of Keywords

Current

- `Type:`
- `Example:`

The: "Description" is everything before the first statement.

Example:

```txt
/*
Description
....

Example:

- Some examples

.....

Type:
 someType :: Int
*/
```
