# Implexa CI/CD with GitHub Actions

This document outlines the continuous integration and deployment (CI/CD) strategy for Implexa using GitHub Actions. This approach automates building, testing, and releasing the application for all target platforms without requiring dedicated build infrastructure.

## Overview

GitHub Actions provides cloud-based virtual machines (runners) that can build the Tauri application natively on each target platform (Windows, macOS, and Linux). This eliminates the need for complex cross-compilation setups and ensures consistent builds across all platforms.

## Workflow Types

Implexa uses multiple GitHub Actions workflows to handle different stages of development:

1. **CI Workflow**: Runs on every push and pull request
2. **Release Workflow**: Creates production builds and releases
3. **Development Builds**: Creates preview builds of feature branches

## CI Workflow

The CI workflow runs on every push to any branch and on pull requests. It verifies that the code builds correctly and passes all tests.

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [ main, dev ]
  pull_request:
    branches: [ main, dev ]

jobs:
  test-tauri:
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-20.04, windows-latest]

    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
          
      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
          
      - name: Install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
          
      - name: Install frontend dependencies
        run: npm install
        
      - name: Run frontend tests
        run: npm test
        
      - name: Rust tests
        run: cargo test
        
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          args: "--ci"
```

## Release Workflow

The release workflow creates production builds for all platforms and publishes them as GitHub releases. It is triggered when a version tag (e.g., `v1.0.0`) is pushed.

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'
  workflow_dispatch:
    inputs:
      version:
        description: 'Version to release (e.g., 1.0.0)'
        required: true

jobs:
  create-release:
    runs-on: ubuntu-20.04
    outputs:
      release_id: ${{ steps.create-release.outputs.result }}
      version: ${{ steps.get-version.outputs.version }}
      
    steps:
      - uses: actions/checkout@v3
      
      - name: Get version from tag
        id: get-version
        if: startsWith(github.ref, 'refs/tags/')
        run: echo "version=${GITHUB_REF#refs/tags/v}" >> $GITHUB_OUTPUT
        
      - name: Get version from input
        if: github.event_name == 'workflow_dispatch'
        run: echo "version=${{ github.event.inputs.version }}" >> $GITHUB_OUTPUT
        
      - name: Create release
        id: create-release
        uses: actions/github-script@v6
        with:
          script: |
            const { data } = await github.rest.repos.createRelease({
              owner: context.repo.owner,
              repo: context.repo.repo,
              tag_name: `v${process.env.VERSION}`,
              name: `Implexa v${process.env.VERSION}`,
              body: 'See the assets to download this version and install.',
              draft: true,
              prerelease: false
            })
            return data.id
        env:
          VERSION: ${{ steps.get-version.outputs.version }}

  build-tauri:
    needs: create-release
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-20.04, windows-latest]
        include:
          - platform: macos-latest
            target: universal-apple-darwin
          - platform: ubuntu-20.04
            target: x86_64-unknown-linux-gnu
          - platform: windows-latest
            target: x86_64-pc-windows-msvc
            
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
          
      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
          
      - name: Install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
          
      - name: Install frontend dependencies
        run: npm install
        
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          releaseId: ${{ needs.create-release.outputs.release_id }}
          
  publish-release:
    runs-on: ubuntu-20.04
    needs: [create-release, build-tauri]
    steps:
      - name: Publish release
        uses: actions/github-script@v6
        with:
          script: |
            github.rest.repos.updateRelease({
              owner: context.repo.owner,
              repo: context.repo.repo,
              release_id: ${{ needs.create-release.outputs.release_id }},
              draft: false
            })
```

## Development Builds Workflow

For feature branches or development builds, a separate workflow creates preview builds that can be shared with team members.

```yaml
# .github/workflows/dev-build.yml
name: Development Build

on:
  workflow_dispatch:
    inputs:
      branch:
        description: 'Branch to build'
        required: true
        default: 'dev'

jobs:
  build-dev:
    strategy:
      fail-fast: false
      matrix:
        platform: [macos-latest, ubuntu-20.04, windows-latest]
        
    runs-on: ${{ matrix.platform }}
    steps:
      - uses: actions/checkout@v3
        with:
          ref: ${{ github.event.inputs.branch }}
          
      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18
          cache: 'npm'
          
      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
          
      - name: Install dependencies (ubuntu only)
        if: matrix.platform == 'ubuntu-20.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libappindicator3-dev librsvg2-dev patchelf
          
      - name: Install frontend dependencies
        run: npm install
        
      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: dev-${{ github.event.inputs.branch }}-${{ github.run_id }}
          releaseName: 'Development Build (${{ github.event.inputs.branch }})'
          releaseBody: 'Development build from branch ${{ github.event.inputs.branch }}. This is not a production release.'
          prerelease: true
          args: "--config tauri.dev.conf.json"
```

## Platform-Specific Requirements

GitHub Actions runners automatically handle most platform-specific dependencies, but some special configurations are needed:

### Linux (Ubuntu)
- Requires specific system packages: `libgtk-3-dev`, `libwebkit2gtk-4.0-dev`, etc.
- These are installed via `apt-get` in the workflow

### macOS
- Universal binary support (Intel + Apple Silicon) is configured in the Tauri configuration file
- Code signing is handled via environment secrets for production builds

### Windows
- MSVC target is used for Windows builds
- Installer customization is defined in the Tauri configuration file

## Setting Up Signing

For production releases, code signing improves security and user experience. To set up signing:

### macOS
1. Add the following secrets to your GitHub repository:
   - `APPLE_CERTIFICATE`: Base64-encoded Apple developer certificate
   - `APPLE_CERTIFICATE_PASSWORD`: Certificate password
   - `APPLE_SIGNING_IDENTITY`: Developer ID Application identity
   - `APPLE_ID`: Apple ID email
   - `APPLE_PASSWORD`: App-specific password for your Apple ID

2. Add signing configuration to the release workflow:
```yaml
- name: Import certificate
  if: matrix.platform == 'macos-latest'
  run: |
    echo $APPLE_CERTIFICATE | base64 --decode > certificate.p12
    security create-keychain -p $KEYCHAIN_PASSWORD build.keychain
    security default-keychain -s build.keychain
    security unlock-keychain -p $KEYCHAIN_PASSWORD build.keychain
    security import certificate.p12 -k build.keychain -P $APPLE_CERTIFICATE_PASSWORD -T /usr/bin/codesign
    security set-key-partition-list -S apple-tool:,apple:,codesign: -s -k $KEYCHAIN_PASSWORD build.keychain
  env:
    APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
    APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
    KEYCHAIN_PASSWORD: ${{ secrets.KEYCHAIN_PASSWORD }}
```

### Windows
1. Add the following secrets to your GitHub repository:
   - `WINDOWS_CERTIFICATE`: Base64-encoded certificate
   - `WINDOWS_CERTIFICATE_PASSWORD`: Certificate password

2. Add signing configuration to the release workflow:
```yaml
- name: Setup Windows signing
  if: matrix.platform == 'windows-latest'
  run: |
    echo ${{ secrets.WINDOWS_CERTIFICATE }} | base64 --decode > certificate.pfx
```

## Benefits of GitHub Actions for Implexa

Using GitHub Actions for Implexa's CI/CD provides several key advantages:

1. **Zero Infrastructure Maintenance**: No need to set up or maintain build servers
2. **Cross-Platform Builds**: Native compilation on all target platforms
3. **Automated Releases**: Streamlined process from code to published release
4. **Consistent Build Environment**: Same environment for every build
5. **Parallel Builds**: Builds for different platforms run simultaneously
6. **Integrated with GitHub**: Seamless integration with issues, PRs, and releases
7. **Free for Public Repositories**: Generous free tier even for private repositories

## Workflow Triggers

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| CI | Push to main/dev, Pull Requests | Verify code builds and passes tests |
| Release | Git tag push, Manual | Create production release builds |
| Dev Build | Manual | Create development/preview builds |

## Secrets Management

Sensitive information such as signing certificates and API keys should be stored as GitHub repository secrets. These are automatically made available to the workflows as environment variables without being exposed in logs.

## Versioning Strategy

Implexa follows Semantic Versioning (SemVer):
- **Major version** (x.0.0): Incompatible API changes
- **Minor version** (0.x.0): New features, backward compatible
- **Patch version** (0.0.x): Bug fixes, backward compatible

Version numbers are maintained in:
1. `package.json` for the frontend
2. `Cargo.toml` for the Rust backend
3. `tauri.conf.json` for the Tauri configuration

The release workflow extracts the version from the Git tag or manual input.

## Conclusion

GitHub Actions provides a powerful, maintenance-free CI/CD solution for Implexa that handles the complexities of cross-platform building and packaging. By using these workflows, the team can focus on development while ensuring consistent, reliable builds across all target platforms.
