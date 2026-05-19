# Auth Improvements — W7

Reference: `/.../W5-Fullstack/D23-Auth/⭐-1-fullstack-auth`

---

## 1. Missing Server-Side Auth Middleware (Critical)

**Problem:** crust_ai only protects routes client-side via `ProtectedRoute` Leptos component.
Client-side redirect = flash of unprotected content + server fns reachable without guard.

**Reference has:** `server/src/middleware/authentication.rs` — Tower `Layer` + `Service` impl that intercepts every HTTP request before Leptos renders.

```
PROTECTED_PATHS: ["/"]     → all routes protected (every path starts with /)
PUBLIC_PATHS:    ["/login", "/api"]  → exempted from redirect
```

Logic:
- Authenticated user hits `/login` → redirect to `/`
- Unauthenticated user hits protected path → redirect to `/login`
- Unauthenticated user hits `/` or `/api/*` → allowed through

**What to add:**

`server/src/middleware/mod.rs`:
```rust
pub mod authentication;
```

`server/src/middleware/authentication.rs`:
```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use axum::body::Body;
use axum::response::{IntoResponse, Redirect, Response};
use http::Request;
use tower::{Layer, Service};

use crate::AuthSession;

const PROTECTED_PATHS: [&str; 1] = ["/"];
const PUBLIC_PATHS: [&str; 2] = ["/login", "/api"];

#[derive(Clone, Default)]
pub struct AuthMiddlewareLayer;

impl AuthMiddlewareLayer {
    pub const fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for AuthMiddlewareLayer {
    type Service = AuthMiddleware<S>;
    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for AuthMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        Box::pin(async move {
            let path = req.uri().path();

            let is_authenticated = match req.extensions().get::<AuthSession>() {
                Some(session) => session.is_authenticated(),
                None => false,
            };

            if should_redirect_to_home(path, is_authenticated) {
                return Ok(Redirect::to("/").into_response());
            }

            if should_redirect_to_auth(path, is_authenticated) {
                return Ok(Redirect::to("/login").into_response());
            }

            inner.call(req).await
        })
    }
}

fn should_redirect_to_home(path: &str, is_authenticated: bool) -> bool {
    path == "/login" && is_authenticated
}

fn should_redirect_to_auth(path: &str, is_authenticated: bool) -> bool {
    if is_authenticated {
        return false;
    }
    if path == "/" {
        return false;
    }
    !PUBLIC_PATHS.iter().any(|p| path.starts_with(p))
        && PROTECTED_PATHS.iter().any(|p| path.starts_with(p))
}
```

Wire into router in `build_app_router.rs` — add `.layer(AuthMiddlewareLayer::new())` ABOVE `AuthSessionLayer`:
```rust
Ok(Router::new()
    .route("/api/{*fn_name}", get(server_fn_handler).post(server_fn_handler))
    .leptos_routes_with_handler(routes, get(leptos_routes_handler))
    .layer(AuthMiddlewareLayer::new())          // ← add this
    .layer(AuthSessionLayer::<User, Uuid, SessionPgPool, PgPool>::new(Some(pool.clone()))
        .with_config(auth_config))
    .layer(SessionLayer::new(session_store))
    .fallback(file_and_error_handler)
    .with_state(app_state))
```

Note: move `.fallback()` AFTER `.layer()` calls so static file handler also runs through auth layers.

---

## 2. Session Pool — Use Separate Pool

**Problem:** crust_ai passes `pool.clone()` to both app state and session store. Reference uses a dedicated `PgPool::connect(...)` for sessions to avoid `search_path` schema conflicts.

**Current (crust_ai):**
```rust
let session_store = SessionStore::<SessionPgPool>::new(Some(pool.clone().into()), session_config).await?;
```

**Fix:**
```rust
let session_pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
let session_store = SessionStore::<SessionPgPool>::new(Some(session_pool.into()), session_config).await?;
```

---

## 3. Session Table Name — Use Env Var

**Problem:** Hardcoded `"sessions"` is inflexible and environment-unaware.

**Current:**
```rust
let session_config = SessionConfig::default()
    .with_table_name("sessions")
    .with_lifetime(chrono::Duration::hours(2));
```

**Fix:**
```rust
let session_config = SessionConfig::default()
    .with_table_name(
        std::env::var("SESSION_TABLE_NAME").expect("SESSION_TABLE_NAME must be set")
    );
```

Add `SESSION_TABLE_NAME=sessions` to `.env`.

---

## 4. Router Layer Order — `.fallback()` Position

**Problem:** crust_ai places `.fallback()` before `.layer()` calls. In Axum, layers wrap in reverse order — fallback added before layers is NOT wrapped by those layers. Static file handler bypasses session/auth entirely.

**Current:**
```rust
.fallback(file_and_error_handler)
.layer(AuthSessionLayer::...)
.layer(SessionLayer::...)
```

**Fix:** move fallback after layers (see item 1 code above).

---

## 5. Add Tests for Auth Middleware Logic

Reference has unit tests covering redirect logic edge cases:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_redirect_to_home() {
        assert!(!should_redirect_to_home("/login", false));
        assert!(should_redirect_to_home("/login", true));
        assert!(!should_redirect_to_home("/dashboard", false));
        assert!(!should_redirect_to_home("/dashboard", true));
    }

    #[test]
    fn test_should_redirect_to_auth() {
        assert!(should_redirect_to_auth("/dashboard", false));
        assert!(!should_redirect_to_auth("/dashboard", true));
        assert!(!should_redirect_to_auth("/login", false));
        assert!(!should_redirect_to_auth("/api/user", false));
        assert!(should_redirect_to_auth("/api/protected", false));
    }
}
```

---

## 6. `is_active` Not Checked at Login

**Problem:** `login` fn calls `auth.login_user()` without checking `user.is_active`. Disabled/banned accounts can still authenticate.

**Fix — add before `auth.login_user()` in `server_fns.rs`:**
```rust
if !user.is_active {
    return Err(ServerFnError::new("Account is disabled"));
}
```

---

## 7. `failed_login_attempts` / `locked_until` — Dead DB Columns

**Problem:** Both fields exist on `User` and in schema but `login` fn never reads or writes them. Brute-force protection is scaffolded but not wired.

**Fix — in `login` fn:**
```rust
// Before password check — reject locked accounts
if let Some(locked_until) = user.locked_until {
    if locked_until > time::OffsetDateTime::now_utc() {
        return Err(ServerFnError::new("Account temporarily locked"));
    }
}

// On wrong password — increment counter, lock after 5 attempts
if !pwhash::bcrypt::verify(&password, &user.password_hash) {
    sqlx::query!(
        "UPDATE users SET
            failed_login_attempts = failed_login_attempts + 1,
            locked_until = CASE
                WHEN failed_login_attempts + 1 >= 5
                THEN NOW() + INTERVAL '15 minutes'
                ELSE locked_until
            END
        WHERE unid = $1",
        user.unid
    )
    .execute(&appstate.pool)
    .await
    .ok();
    return Err(ServerFnError::new("Invalid email or password"));
}

// On success — reset counter
sqlx::query!(
    "UPDATE users SET failed_login_attempts = 0, locked_until = NULL WHERE unid = $1",
    user.unid
)
.execute(&appstate.pool)
.await
.ok();
```

---

## 8. `last_login_at` Never Updated

**Problem:** Column exists on `User`, always `NULL` after login.

**Fix — add after successful password verify in `login` fn:**
```rust
sqlx::query!(
    "UPDATE users SET last_login_at = NOW() WHERE unid = $1",
    user.unid
)
.execute(&appstate.pool)
.await
.ok();
```

---

## 9. `current_user` Resource Never Provided in App Context (Critical)

**Problem:** `protected_route.rs` does:
```rust
use_context::<Resource<Result<Option<User>, ServerFnError>>>()
    .expect("current_user resource missing from context") // panics at runtime
```

But `app.rs` never provides this resource → **runtime panic on every protected route**.

**Fix — in `App` component (`app/src/app.rs`), add:**
```rust
let current_user = Resource::new(|| (), |_| get_current_user());
provide_context(current_user);
```

---

## Summary Table

| Issue | Severity | File(s) |
|-------|----------|---------|
| No server-side auth middleware | **Critical** | `server/src/middleware/authentication.rs` (new), `server/src/app_router/build_app_router.rs` |
| `current_user` resource not provided in context | **Critical** | `app/src/app.rs` |
| `is_active` not checked at login | **High** | `app/src/domain/auth/server_fns.rs` |
| `failed_login_attempts` / `locked_until` unused | **High** | `app/src/domain/auth/server_fns.rs` |
| Shared session pool | Medium | `server/src/app_router/build_app_router.rs` |
| `.fallback()` before layers | Medium | `server/src/app_router/build_app_router.rs` |
| `last_login_at` never updated | Low | `app/src/domain/auth/server_fns.rs` |
| Hardcoded session table name | Low | `server/src/app_router/build_app_router.rs`, `.env` |
| No middleware unit tests | Low | `server/src/middleware/authentication.rs` |
