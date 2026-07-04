# CORS — Credentials Support (already satisfied)

## Context

CORS is **already fully wired up and already supports credentials.**

`apps/api/src/main.rs` applies `CorsLayer::very_permissive()` to the router, and `tower-http`'s `cors` feature is enabled in `apps/api/Cargo.toml`. Verified against `tower-http 0.6.11` source — `very_permissive()` is defined as:

```rust
pub fn very_permissive() -> Self {
    Self::new()
        .allow_credentials(true)                       // ✅ credentials already on
        .allow_headers(AllowHeaders::mirror_request())
        .allow_methods(AllowMethods::mirror_request())
        .allow_origin(AllowOrigin::mirror_request())    // echoes request Origin, NOT "*"
}
```

**Key point:** `AllowOrigin::mirror_request()` echoes the request's specific `Origin` header value back as `Access-Control-Allow-Origin`. It does **not** emit the literal `*` wildcard. The CORS spec only forbids `*` together with `Allow-Credentials: true` — mirroring a concrete origin is spec-legal and browser-accepted. Combined with `allow_credentials(true)`, **cookie-based cross-origin auth (the `axum-extra` session cookies) already works today.**

The user's chosen goal — "credentials only" — is **already met by existing code. Nothing needs to change.**

## The one remaining (optional) gap: origin allow-listing

`very_permissive()` mirrors _any_ origin. That means any website can make credentialed requests to the API. For a deployed frontend this is a security concern — but it is **not** required for credentials to work. Locking origins down is a hardening measure, not a correctness fix.

If origin restriction is later desired, it's a one-function change in `main.rs`:

```rust
fn cors_layer() -> CorsLayer {
    match std::env::var("CORS_ORIGINS") {
        Ok(list) if !list.trim().is_empty() => CorsLayer::new()
            .allow_credentials(true)
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(list.split(',').filter_map(|s| s.trim().parse().ok()).collect::<Vec<_>>()),
        _ => CorsLayer::very_permissive(), // dev: permissive
    }
}
```

That's deferred until there's a real production frontend origin to lock down.

## Conclusion

No code change. CORS + credentials already work. Closing the plan.
