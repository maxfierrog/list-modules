# List-Modules Procedural Macro

**WARNING: This crate is domain-specific. The only thing that makes it so
is that you cannot name one of the directory items you are trying to list
"archetypes.rs" (a module folder named "archetypes" is fine). I will try
to fix this ASAP.**

This crate creates a constant string slice list of all the module names
which are children of the crate of the module folder it was called from.
Note that it will only have the desired function if it is called from the
`mod.rs` file of a module structured in a folder (not a file).

For example, calling this macro with `list_modules::here!()` inside `mod.rs` 
in the following file tree:

```
parent/
    mod.rs
    child_1.rs
    child_2/
        mod.rs
        internal.rs
        other_internal/
            ...
        ...
    child_3.rs
    child_4.rs
    ...
    child_N/
        mod.rs
```

...will result in the following list expansion:

```rust
pub const LIST: [&str; N] = [
    "child_1",
    "child_2",
    "child_3",
    ...
    "child_n",
];
```

Note that this is the only guaranteed behavior.
