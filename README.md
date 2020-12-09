# openbsd

Rust bindings for OpenBSD's pledge(2) and unveil(2).

## Usage

### Pledge

#### Macro syntax

```rust
use openbsd::pledge;

pledge!("stdio rpath exec")?; // only make promises
pledge!(_, "stdio rpath")?; // only make execpromises
pledge!("stdio", "stdio")?; // make both

assert!(pledge!("wpath").is_err()); // cannot increase permissions
```

#### Function syntax

```rust
use openbsd::pledge::{pledge, pledge_promises, pledge_execpromises};

pledge_promises("stdio rpath exec")?; // only make promises
pledge_execpromises("stdio rpath")?; // only make execpromises
pledge("stdio", "stdio")?; // make both

assert!(pledge_promises("wpath").is_err()); // cannot increase permissions
```

### Unveil

#### Macro syntax

```rust
use openbsd::unveil;

unveil!("/path/to/file", "rw")?;
unveil!("/path/to/another/file", "r")?;

unveil!(); // disable further calls to unveil
assert!(unveil!("/", "rwxc").is_err());
```

#### Function syntax

```rust
use openbsd::unveil;

unveil("/path/to/file", "rw")?;
unveil("/path/to/another/file", "r")?;

unveil::disable(); // disable further calls to unveil
assert!(unveil("/", "rwxc").is_err());
```
