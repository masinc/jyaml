# JYAML Rust Implementation - Release Guide

This document outlines the process for releasing new versions of the JYAML Rust implementation.

## Release Process Overview

1. [Prepare the Release](#1-prepare-the-release)
2. [Update Version Numbers](#2-update-version-numbers)
3. [Update Documentation](#3-update-documentation)
4. [Run Pre-Release Tests](#4-run-pre-release-tests)
5. [Create Git Tag](#5-create-git-tag)
6. [Automated Release](#6-automated-release)
7. [Post-Release Verification](#7-post-release-verification)

## 1. Prepare the Release

### Check Current State
```bash
cd implementations/rust
git status
git pull origin main
```

### Review Changes
- Check all merged PRs since last release
- Ensure all tests pass: `cargo test`
- Verify test suite compliance: `cargo run --example test_file -- ../../test-suite/valid/basic/hello.jyml`

## 2. Update Version Numbers

### Update Cargo.toml
```toml
[package]
version = "X.Y.Z"  # Update to new version
```

### Version Guidelines
Follow [Semantic Versioning](https://semver.org/):
- **MAJOR** (X): Breaking changes to API
- **MINOR** (Y): New features, backward compatible
- **PATCH** (Z): Bug fixes, backward compatible

### Examples:
- `0.3.0` → `0.3.1`: Bug fix
- `0.3.0` → `0.4.0`: New features
- `0.3.0` → `1.0.0`: Breaking changes

## 3. Update Documentation

### Update CHANGELOG.md
Add new version entry at the top:
```markdown
## [X.Y.Z] - YYYY-MM-DD

### Added
- New features

### Changed
- Modified behavior

### Fixed
- Bug fixes

### Removed
- Deprecated features
```

### Update README.md (if needed)
- Version numbers in examples
- New feature documentation
- Installation instructions

## 4. Run Pre-Release Tests

### Local Testing
```bash
# Run all tests
cargo test --all-features

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check formatting
cargo fmt --all -- --check

# Test against JYAML test suite
cargo run --example test_file -- ../../test-suite/valid/basic/hello.jyml
```

### Manual CI Test (Optional)
Go to GitHub Actions → "Rust CI" → "Run workflow" to manually trigger CI.

## 5. Create Git Tag

### Commit Changes
```bash
git add .
git commit -m "Release version X.Y.Z

- Updated version in Cargo.toml
- Updated CHANGELOG.md with release notes
- [Brief summary of changes]

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"

# Push changes first
git push origin main
```

### Wait for CI to Pass
After pushing to main, **wait for CI to complete successfully** before creating the release tag:

1. Go to GitHub Actions: https://github.com/masinc/jyaml/actions
2. Find the "Rust CI" workflow for your latest commit
3. Wait for all jobs to complete ✅
4. **Do not proceed if any tests fail** ❌

```bash
# Optional: Monitor CI status from command line
gh run watch
```

### Create and Push Tag
```bash
# Create annotated tag
git tag -a rust-vX.Y.Z -m "JYAML Rust vX.Y.Z"

# Push commit and tag
git push origin main
git push origin rust-vX.Y.Z
```

### Tag Naming Convention
- Format: `rust-vX.Y.Z`
- Examples: `rust-v0.3.0`, `rust-v0.3.1`, `rust-v1.0.0`

## 6. Automated Release

Once the tag is pushed, GitHub Actions will automatically:

1. **Run Tests**: Full test suite including JYAML compliance tests
2. **Build Binaries**: Cross-platform builds for Linux, Windows, macOS
3. **Create GitHub Release**: With changelog and download links
4. **Publish to crates.io**: Make the crate available via `cargo install`

### Monitor Release Progress
1. Go to GitHub Actions tab
2. Watch "Rust Release" workflow
3. Check for any failures and fix if needed

## 7. Post-Release Verification

### Verify GitHub Release
- Check release is created: https://github.com/masinc/jyaml/releases
- Verify binary downloads work
- Confirm changelog is correctly formatted

### Verify crates.io Publication
- Check crate page: https://crates.io/crates/jyaml
- Test installation: `cargo install jyaml --version X.Y.Z`

### Test Installation
```bash
# Create test project
cargo new jyaml-test
cd jyaml-test

# Add dependency
echo 'jyaml = "X.Y.Z"' >> Cargo.toml

# Test basic usage
cat > src/main.rs << 'EOF'
use jyaml::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jyaml_text = r#"
    {
      "name": "Alice",
      "age": 30,
    }
    "#;
    
    let value = parse(jyaml_text)?;
    println!("{:#?}", value);
    Ok(())
}
EOF

cargo run
```

## Manual Release (Alternative)

If automated release fails, you can trigger manually:

### Using GitHub Web Interface
1. Go to Actions → "Rust Release"
2. Click "Run workflow"
3. Enter version number (e.g., `0.3.1`)
4. Click "Run workflow"

### Using gh CLI
```bash
gh workflow run rust-release.yml -f version=X.Y.Z
```

## Troubleshooting

### Common Issues

#### Version Mismatch Error
**Problem**: Release fails with "Version mismatch" error
**Solution**: Ensure Cargo.toml version matches the git tag version

#### crates.io Authentication Error  
**Problem**: Publishing to crates.io fails
**Solution**: Check `CARGO_REGISTRY_TOKEN` secret is set in repository settings

#### Test Failures
**Problem**: Pre-release tests fail
**Solution**: Fix failing tests before proceeding with release

#### Binary Build Failures
**Problem**: Cross-compilation fails for some targets
**Solution**: Check GitHub Actions logs and fix platform-specific issues

### Recovery Steps

#### If Release Fails After Tag Creation
1. Delete the tag: `git tag -d rust-vX.Y.Z && git push origin :refs/tags/rust-vX.Y.Z`
2. Fix the issues
3. Create new tag with patch version: `rust-vX.Y.Z+1`

#### If crates.io Publishing Fails
1. The GitHub release will still be created
2. Manually publish: `cargo publish` from `implementations/rust/`
3. Or re-run just the publish job in GitHub Actions

## Release Checklist

- [ ] All tests pass locally
- [ ] Version updated in Cargo.toml
- [ ] CHANGELOG.md updated
- [ ] Changes committed and pushed to main branch
- [ ] **CI passes on main branch** ✅
- [ ] Git tag created and pushed
- [ ] GitHub Actions release workflow completed successfully
- [ ] GitHub release created with binaries
- [ ] Package published to crates.io
- [ ] Installation tested from crates.io
- [ ] Release announcement (if needed)

## Next Steps After Release

1. **Update implementations/README.md** if this was a major release
2. **Update project documentation** if new features were added
3. **Consider blog post or announcement** for significant releases
4. **Plan next release** - add issues/features to next milestone