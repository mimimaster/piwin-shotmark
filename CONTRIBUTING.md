# Contributing

Small repo, three rules:

1. **One concern per commit.** Refactors stay separate from behaviour changes.
2. **Claim only what you ran.** If a change touches Windows or Linux, say in the
   PR what you executed and on which OS; `cargo check --target ...` is not a
   smoke test.
3. **Run this before asking for review:**

```bash
cargo test --manifest-path src-tauri/Cargo.toml
npm run build
```

Keep production files under ~400 lines where it is natural. The annotation model
(`src/screen-capture/annotation-model.ts`) is deliberately dependency-free —
keep it that way.
