## Dependencies
specifier 0.8.5 is shorthand for ^0.8.5 i.e. 0.8.5 <= version < 0.9.0. Any version 0.9.0 or greater is not guaranteed to have same API as 0.8.5

### Note
Cargo knows if a program has not been changed and hence does not recompile it and simply exits.

## Updating a Crate to a New Version
```bash
cargo update
```
This ignores the Cargo.lock file and figures out the latest version of the dependencies that still satisfy the version requirements in the Cargo.toml file.
Cargo will then write those versions to Cargo.lock file.
In this case it will look for any version greater than 0.8.5 but less than 0.9.0.
