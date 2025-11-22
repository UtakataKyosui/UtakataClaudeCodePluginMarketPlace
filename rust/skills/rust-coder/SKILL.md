---
name: rust-coding-skill
description: Helps Claude write idiomatic Rust code with structs, impl blocks, macros, and best practices for build speed and documentation.
---

# Rust Coding Skill

## Instructions
1. Understand the user request: determine if it involves structs, impls, traits, or macros.  
2. Plan data structures and relationships, considering ownership, borrowing, and lifetimes.  
3. Write idiomatic Rust using structs, impl blocks, and macros.  
4. Apply best practices:
   - Add `///` comments for structs and fields.
   - Place `impl` blocks directly under their corresponding structs or enums.
   - Order methods logically (CRUD or by related functionality).
   - Add empty lines between methods.
   - Use macros for repetitive patterns or derive attributes.
5. Optimize build speed when relevant:
   - On Linux, consider `mold` linker via `.cargo/config.toml`.
   - Use `sccache` to cache compiler artifacts and accelerate rebuilds.
6. Provide explanations and alternatives for design choices.
