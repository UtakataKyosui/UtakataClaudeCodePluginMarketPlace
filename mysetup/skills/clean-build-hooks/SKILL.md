---
name: clean-build-hooks
description: Performs a clean build of the integrated Claude Code hooks by removing all build artifacts and rebuilding from scratch. Use this skill when encountering build errors, after updating Rust toolchain, when troubleshooting mysterious build issues, or when the user explicitly requests a clean rebuild. This is more thorough than the regular build-hooks skill.
---

# Clean Build Hooks

Performs a clean build of the integrated Claude Code hooks by removing all build artifacts and compiling from scratch.

## When to Use This Skill

- When encountering unexplained build errors
- After updating the Rust toolchain or dependencies
- When build artifacts may be corrupted
- After making significant changes to the source code
- When the user explicitly requests a clean rebuild
- When regular builds are failing but source code appears correct

## Instructions

1. **Navigate to the hooks directory**:
   ```bash
   cd ${CLAUDE_PLUGIN_ROOT}/hooks/claude-hooks/
   ```

2. **Clean all build artifacts**:
   ```bash
   cargo clean
   ```
   This removes the entire `target/` directory including all compiled artifacts, dependencies, and cached builds.

3. **Perform a fresh release build**:
   ```bash
   cargo build --release
   ```

4. **Verify the build**:
   - Check that the binary was created at `target/release/integrated-claude-hooks`
   - Display the binary size: `ls -lh target/release/integrated-claude-hooks`
   - Confirm successful completion

5. **Report the results**:
   - Indicate that a clean build was performed
   - Show the new binary path and size
   - Report any errors encountered during the process

## Expected Output

After a clean build, the binary should be recreated at:
```
${CLAUDE_PLUGIN_ROOT}/hooks/claude-hooks/target/release/integrated-claude-hooks
```

Typical binary size: ~3.0MB (optimized release build)

Build time: Longer than incremental builds (2-15 seconds depending on system)

## Difference from Regular Build

| Aspect | Regular Build (`build-hooks`) | Clean Build (`clean-build-hooks`) |
|--------|-------------------------------|-----------------------------------|
| Speed | Fast (incremental) | Slower (full recompilation) |
| Artifacts | Reuses cached builds | Removes all cached builds |
| Use Case | Normal updates | Troubleshooting, major changes |
| Dependencies | Only recompiles changed code | Recompiles everything |

## Troubleshooting

If the clean build still fails:
- Check Rust installation: `rustc --version` and `cargo --version`
- Update Rust toolchain: `rustup update`
- Check for disk space: `df -h`
- Review error messages for missing system libraries
- Verify source code has no syntax errors

Common issues:
- **Out of disk space**: Clean build needs more temporary space
- **Missing system libraries**: On macOS, ensure Xcode Command Line Tools are installed
- **Permission issues**: Ensure write access to the project directory
