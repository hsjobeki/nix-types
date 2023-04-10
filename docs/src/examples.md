# Examples

This section contains some illustrative examples that show how to practically apply the specification in concrete use-case-scenarios.

## Short preview of the basic syntax

```nix
# lib.lists.any

/* 
    Type: 
       ( a -> Bool ) -> [ a ] -> Bool
        └──────────┘  ↑          ↑           
             |        |          └── final return value is a Bool
             |        └── The function returns a function that takes a list of type 'a'      
             └── function that takes an 'a' and returns True or False (Boolean)

*/
any = builtins.any or (pred: foldr (x: y: if pred x then true else y) false);
```
