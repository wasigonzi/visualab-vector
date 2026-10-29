# Visualab Vector branding fork

This fork changes visible product strings, default theme colours and application icons. The artwork is provisional; an official Visualab logo has not been supplied. Internal package names, format IDs, file extensions, signing configuration, updater feeds, platform IDs and preferences locations are deliberately unchanged in this first source pass. Install alongside an upstream copy only after those identities have been separated.

## Release checklist

- Run the upstream Cargo quality gates and inspect the application on each target OS.
- Supply official artwork if desired.
- Separate platform identifiers, settings directories, file associations and installer upgrade codes.
- Replace updater feeds and community links where they still refer to the upstream project.
- Configure Visualab signing certificates and independent release workflows.
- Audit attribution and package contents.

No signed installers or verified functional preview are delivered yet. Existing upstream feature limitations remain.

## Automated verification

`cargo clippy -p vectorcraft-ui-egui --lib -- -D warnings`: passed.

Formatting and dependency-layer checks were run; Vector and Effects asset-attribution checks passed. Full workspace CI and runtime / screenshot verification are incomplete. The new regression tests have not been executed. These changes are a draft for review, not a validated release.
