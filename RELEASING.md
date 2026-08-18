# Releasing crates

Each crate in this workspace is versioned and published independently.

## One-time repository setup

Create the GitHub Actions repository secret:

- `CARGO_REGISTRY_TOKEN`: crates.io API token with permission to publish the `xuan-*` crates

Do not commit registry credentials to the repository.

## Release gates

Before publishing a crate:

1. Merge the intended code and version bump into `main`.
2. Confirm CI passes formatting, Clippy with warnings denied, and workspace tests.
3. Confirm the crate's source and third-party provenance are compatible with the repository's `MIT OR Apache-2.0` license.
4. Review `cargo package --list` / `cargo publish --dry-run` output for accidental files or metadata problems.
5. Confirm the requested version has not already been published. crates.io versions are immutable.

## Publish

Use **Actions -> Publish crate -> Run workflow** from `main`.

Select one package and enter the exact manifest version. The workflow validates the version, runs the full quality gate, performs `cargo publish --dry-run`, publishes the crate, and creates a package-scoped Git tag:

```text
xuan-cosmology-v0.1.0
xuan-calendar-v0.1.0
```

For the initial release, publish in this order:

1. `xuan-cosmology` `0.1.0`
2. `xuan-calendar` `0.1.0`

`xuan-calendar` has a crates.io version dependency on `xuan-cosmology`, so the lower-level crate must be available in the registry first.

## Future releases

Bump only the crate that actually changes. If a breaking or otherwise required `xuan-cosmology` update changes `xuan-calendar`'s dependency requirement, publish the new `xuan-cosmology` version first, update/test `xuan-calendar`, then publish its new version.
