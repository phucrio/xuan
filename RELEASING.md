# Releasing crates

Each crate in this workspace is versioned and released independently.

## Tag convention

Release tags must follow:

```text
<crate-name>-v<semver>
```

Examples:

```text
xuan-cosmology-v0.1.0
xuan-calendar-v0.1.0
```

Do not use workspace-wide tags such as `0.1.0` or `v0.1.0`.

## Repository setup

The publish workflow expects the repository secret:

- `CARGO_REGISTRY_TOKEN`: crates.io API token with permission to publish the `xuan-*` crates.

## Release flow

A release PR should update the intended crate version and its changelog. After the release changes are merged into `main`, `.github/workflows/publish.yml` runs the full quality gate and publishes any crate whose `<crate-name>-v<semver>` tag does not already exist.

For each released crate, the workflow:

1. runs formatting, Clippy with warnings denied, and workspace tests;
2. runs `cargo publish --dry-run`;
3. publishes the crate to crates.io;
4. creates and pushes the annotated `<crate-name>-v<semver>` tag;
5. creates a GitHub Release from that tag.

The workflow can also be re-run manually from **Actions -> Publish crates**. Existing release tags are treated as completed releases and are skipped.

## Dependency order

`xuan-calendar` depends on the registry version of `xuan-cosmology`, so releases are processed in this order:

1. `xuan-cosmology`
2. wait until that version is visible in the crates.io index
3. `xuan-calendar`

For the initial release this produces:

```text
xuan-cosmology-v0.1.0
xuan-calendar-v0.1.0
```

## Future releases

Bump only the crate that actually changes. If `xuan-cosmology` changes in a way that requires a new `xuan-calendar` dependency range, release the new `xuan-cosmology` version first, update and test `xuan-calendar`, then release its new version.
