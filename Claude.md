# Rules

* When adding dependencies, add them in alphabetical order.
* Before making any Rust code changes, disable the rust-analyzer extension to prevent server crashes:
  ```bash
  code --disable-extension rust-lang.rust-analyzer
  ```
  After completing all Rust code changes, re-enable the extension:
  ```bash
  code --enable-extension rust-lang.rust-analyzer
  ```
* Comments
    * Avoid superfluous, trivial comments. Only add comments to explain *why*s that are non-obvious, or particularly complex and non-trivial logic.
    * The exception is rustdoc comments. rustdoc comments should be concise, clear, and comprehensive.
    * If you update the logic of a function/variable, check the rustdoc comments and update them appropriately.
