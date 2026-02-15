# Release Checklist

This checklist documents the minimal steps to validate and publish a release of `1install`.

Developer quick-flow:

- [ ] Update `CHANGELOG.md` with user-facing changes.
- [ ] Bump the version in `Cargo.toml` and `pyproject.toml` as appropriate.
- [ ] Ensure `cargo test` passes locally.
- [ ] Run integration checks: `scripts/tests/run_integration_tests.sh`.
- [ ] Verify the `scripts/install.sh` and `scripts/install.ps1` bootstrappers work in a fresh environment (or ci runners).
- [ ] Validate shims: `1i shims path` and `1i shims setup` produce the expected instructions and files.
- [ ] Confirm `1i install node` (or similar common packages) works on a test VM/container for target OS.
- [ ] Ensure `docs/VERSION_PLAN.md` is up-to-date with feature statuses.
- [ ] Create annotated git tag: `git tag -a vX.Y.Z -m "Release vX.Y.Z"` and push.

Publishing:

- [ ] Run release CI to build multi-platform artifacts.
- [ ] Draft GitHub Release notes using `CHANGELOG.md` entries.
- [ ] Upload assets and attach checksums/signatures if applicable.

Post-release:

- [ ] Monitor telemetry (opt-in) for regressions.
- [ ] Address any immediate bug reports and ship hotfix if needed.

Notes:

- Use the `RELEASE_CHECKLIST.md` for every release and keep it short and actionable.
