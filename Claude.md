# Rules

* Before making any Rust code changes, disable the rust-analyzer extension to prevent server crashes:
  ```bash
  code --disable-extension rust-lang.rust-analyzer
  ```
  After completing all Rust code changes, re-enable the extension:
  ```bash
  code --enable-extension rust-lang.rust-analyzer
  ```

# Code Style
* NEVER make changes to `/clients` or `openapi.json` unless specifically directed to. Code generation tools will handle this for us.
* Keep mutation or state manipulation as isolated as possible
* Keep functions short and simple. If a function is too long, split it into shorter, self documenting functions.
* Avoid verbose fully qualified module syntax. Prefer imports instead.
* Avoid non-deterministic function calls inside of `*Operation::apply_real(...)` calls. This includes things like
  * Generating random values (uuids, sampling, etc.)
  * Generating timestamps
  * Any data non-deterministically generated inside `*Operation::apply_real(...)` should instead be passed into the Operation before apply_real step.
* `#[instrument]` any function that performs IO (network or disk).
* Comments
    * Avoid superfluous, trivial comments. Only add comments to explain *why*s that are non-obvious, or particularly complex and non-trivial logic.
    * The exception is rustdoc comments. rustdoc comments should be concise, clear, and comprehensive.
    * If you update the logic of a function/variable, check the rustdoc comments and update them appropriately.

# Commands
* `just lint`: detect and possibly fix any style warnings.
* `just test <optional name of test>` to run tests.
