# General Improvements — W7

| Priority    | #  | Issue                                                 | Location                                          |
|-------------|-----|-------------------------------------------------------|---------------------------------------------------|
| 🔴 High     | 1   | No `clippy.toml`                                      | workspace root                                    |
| 🔴 High     | 2   | No `[workspace.lints]`                                | `Cargo.toml`                                      |
| 🔴 High     | 3   | `dotenv` → `dotenvy`                                  | `Cargo.toml`, `main.rs`                           |
| 🔴 High     | 4   | `.expect()` / `.unwrap()` in `main.rs`                | `server/src/main.rs`                              |
| 🔴 High     | 5   | `use_context().expect()` in components                | `app_sidenav.rs`, `page_chat.rs`, `page_home.rs`  |
| 🔴 High     | 6   | `password_hash` field is `pub`                        | `app/src/domain/auth/model.rs`                    |
| 🟡 Medium   | 7   | `sqlx::query!` instead of `query_as!`                 | `app/src/domain/auth/server_fns.rs`               |
| 🟡 Medium   | 8   | `ServerFnError` inconsistent variants                 | `server_fns.rs`, `app_state.rs`                   |
| 🟡 Medium   | 9   | `fallback.rs` unwraps                                 | `server/src/fallback.rs`                          |
| 🟡 Medium   | 10  | `chrono` dep redundant                                | `Cargo.toml`, `build_app_router.rs`               |
| 🟡 Medium   | 11  | `sqlx` missing `default-features = false` + `migrate` | `Cargo.toml`                                      |
| 🟡 Medium   | 12  | No `.cargo/config.toml`                               | workspace root                                    |
| 🟢 Low      | 13  | `EnvFilter` no fallback                               | `server/src/main.rs`                              |

---

## 1. Add `clippy.toml`

Create `clippy.toml` at workspace root:

```toml
allow-unwrap-in-tests = true
allow-expect-in-tests = true
allow-panic-in-tests = true

disallowed-macros = [
    { path = "sqlx::query", reason = "Use query_as! or query_scalar! for compile-time type safety" },
]
disallowed-methods = [
    { path = "sqlx::query_as", reason = "Use query_as! macro for compile-time type safety" },
    { path = "sqlx::query",    reason = "Use query_as! or query_scalar! macro for compile-time type safety" },
]
```

---

## 2. Add `[workspace.lints]` to `Cargo.toml`

```toml
[workspace.lints.clippy]
# Safety & Correctness
unwrap_used               = "deny"
expect_used               = "deny"
todo                      = "deny"
await_holding_lock        = "deny"
disallowed_macros         = "deny"   # See clippy.toml
disallowed_methods        = "deny"   # See clippy.toml
undocumented_unsafe_blocks = "deny"

# Code Quality
dbg_macro                 = "deny"
print_stdout              = "deny"
print_stderr              = "deny"
redundant_clone           = "warn"
inefficient_to_string     = "warn"
cloned_instead_of_copied  = "warn"
unused_async              = "warn"
needless_pass_by_value    = "warn"
implicit_clone            = "warn"
str_to_string             = "warn"

[workspace.lints.rust]
irrefutable_let_patterns  = "deny"
```

---

## 3. Replace `dotenv` with `dotenvy`

`dotenv` is unmaintained. `dotenvy` is the maintained fork.

`Cargo.toml`:
```toml
# Remove:
dotenv = "0.15"

# Add:
dotenvy = "0.15"
```

`server/src/main.rs`:
```rust
// Remove:
use dotenv::dotenv;

// Add:
use dotenvy::dotenv;
```

---

## 4. Remove `chrono` — Use `time` Only

`chrono` appears only for session lifetime. Replace with `time`:

`Cargo.toml` — remove `chrono` entirely.

`build_app_router.rs`:
```rust
// Remove:
.with_lifetime(chrono::Duration::hours(2))

// Add:
.with_lifetime(time::Duration::hours(2))
```

---

## 5. Fix `sqlx` — Missing `default-features = false` and `migrate` Feature

```toml
# Before:
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "macros", "uuid", "time", "json"] }

# After:
sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio-rustls", "postgres", "macros", "migrate", "uuid", "time", "json"] }
```

---

## 6. Add `.cargo/config.toml` — Build Speed

Create `.cargo/config.toml`:

```toml
[profile.dev]
debug = false
panic = "abort"
split-debuginfo = "unpacked"  # macOS: avoids dsymutil, huge link time savings

[profile.dev.package."*"]
opt-level = 3  # fast dep compilation, cached after first build
```

---

## 7. Fix `main.rs` — Remove All `.expect()` / `.unwrap()`

```rust
// Before:
LogTracer::init().expect("Failed to set logger");
tracing::subscriber::set_global_default(subscriber).expect("Could not set subscriber");
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
axum::serve(listener, app).await.unwrap();

// After:
LogTracer::init()?;
tracing::subscriber::set_global_default(subscriber)
    .map_err(|e| anyhow::anyhow!("Could not set subscriber: {e}"))?;
let database_url = env::var("DATABASE_URL")
    .map_err(|_| anyhow::anyhow!("DATABASE_URL not set"))?;
axum::serve(listener, app).await?;
```

---

## 8. Fix `EnvFilter` — Add Fallback to `info`

```rust
// Before:
.with_env_filter(EnvFilter::from_default_env())

// After:
.with_env_filter(
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
)
```

---

## 9. `use_context().expect()` — Runtime Panics in Components

Multiple components panic at runtime if context is missing. Pattern to fix everywhere:

`app/src/layout/app_sidenav.rs`, `app/src/domain/chat/page_chat.rs`, `app/src/domain/home/page_home.rs`:

```rust
// Before — panics if context not provided:
let sidenav_open = use_context::<RwSignal<bool>>()
    .expect("AppSidenav: sidenav_open context not found");

// After — renders nothing or error view:
let Some(sidenav_open) = use_context::<RwSignal<bool>>() else {
    return view! { <p>"Context missing"</p> }.into_any();
};
```

---

## 10. `User` Struct — `password_hash` Is `pub`

All `User` fields are `pub`, including `password_hash`. Leaks sensitive data if struct is serialized or logged carelessly.

`app/src/domain/auth/model.rs`:

```rust
// Before:
pub struct User {
    pub unid: Uuid,
    pub email: String,
    pub password_hash: String,  // ← dangerous
    ...
}

// After — keep fields private, expose via methods:
pub struct User {
    pub unid: Uuid,
    pub email: String,
    password_hash: String,  // private
    ...
}
```

Or create a separate `UserPublic` / `UserView` struct for serialization that excludes `password_hash`.

---

## 11. `sqlx::query!` Used Instead of `query_as!`

`app/src/domain/auth/server_fns.rs:25`:

```rust
// Before — no compile-time type safety:
sqlx::query!("INSERT INTO users ...")

// After — use query_as! or query_scalar!:
sqlx::query_as!(User, "INSERT INTO users ... RETURNING *")
// or for no return:
sqlx::query!(...) is ok ONLY for statements with no return value
// for SELECT always use query_as! or query_scalar!
```

---

## 12. `ServerFnError` — Inconsistent Variants

Two different patterns used across the codebase. Pick one:

```rust
// Pattern A (used in server_fns.rs):
ServerFnError::new("message")

// Pattern B (used in app_state.rs):
ServerFnError::ServerError("message".into())
```

`ServerFnError::new()` is correct for generic string errors. Remove all `::ServerError(...)` usage, use `::new()` everywhere.

---

## 13. `fallback.rs` — Unwraps on Request Builder

`server/src/fallback.rs`:

```rust
// Before:
let req = Request::builder()
    .uri(uri)
    .body(Body::empty())
    .unwrap();  // panics on invalid URI

// After:
let req = Request::builder()
    .uri(uri)
    .body(Body::empty())
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
```

