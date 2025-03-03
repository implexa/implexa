# Implexa CI/CD Setup Guide

This document provides instructions for setting up and using the Continuous Integration and Continuous Deployment (CI/CD) pipeline for Implexa using GitHub Actions.

## Overview

Implexa uses GitHub Actions for automating the build, test, and release processes. This approach provides:

- Cross-platform builds (Windows, macOS, Linux)
- Automated testing
- Streamlined release process
- No need for dedicated build infrastructure

## Prerequisites

Before setting up the CI/CD pipeline, ensure you have:

1. A GitHub repository for the Implexa project
2. Administrator access to the repository
3. Basic understanding of YAML syntax
4. (Optional) Code signing certificates for production releases

## Workflow Files Structure

Create a `.github/workflows` directory in your repository and add the following workflow files:

1. `ci.yml` - For continuous integration
2. `release.yml` - For creating production releases
3. `dev-build.yml` - For creating development/preview builds

## Setting Up the CI Workflow

The CI workflow runs on every push to main branches and on pull requests to verify that the code builds correctly and passes all tests.

Create `.github/workflows/ci.yml` with the following content:

```yaml
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

## Setting Up the Release Workflow

The release workflow creates production builds for all platforms and publishes them as GitHub releases. It is triggered when a version tag (e.g., `v1.0.0`) is pushed or manually triggered.

Create `.github/workflows/release.yml` with the following content:

```yaml
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

## Setting Up the Development Builds Workflow

For feature branches or development builds, a separate workflow creates preview builds that can be shared with team members.

Create `.github/workflows/dev-build.yml` with the following content:

```yaml
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

## Setting Up Code Signing

For production releases, code signing improves security and user experience. Follow these steps to set up signing for each platform:

### macOS Signing Setup

1. Add the following secrets to your GitHub repository:
   - `APPLE_CERTIFICATE`: Base64-encoded Apple developer certificate
   - `APPLE_CERTIFICATE_PASSWORD`: Certificate password
   - `APPLE_SIGNING_IDENTITY`: Developer ID Application identity
   - `APPLE_ID`: Apple ID email
   - `APPLE_PASSWORD`: App-specific password for your Apple ID
   - `KEYCHAIN_PASSWORD`: A password for the temporary keychain

2. Add the following steps to the `build-tauri` job in the release workflow, before the "Build Tauri app" step:

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

3. Update the `tauri-action` step to include signing configuration:

```yaml
- name: Build Tauri app
  uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
    APPLE_ID: ${{ secrets.APPLE_ID }}
    APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
  with:
    releaseId: ${{ needs.create-release.outputs.release_id }}
```

### Windows Signing Setup

1. Add the following secrets to your GitHub repository:
   - `WINDOWS_CERTIFICATE`: Base64-encoded certificate
   - `WINDOWS_CERTIFICATE_PASSWORD`: Certificate password

2. Add the following steps to the `build-tauri` job in the release workflow, before the "Build Tauri app" step:

```yaml
- name: Setup Windows signing
  if: matrix.platform == 'windows-latest'
  run: |
    echo ${{ secrets.WINDOWS_CERTIFICATE }} | base64 --decode > certificate.pfx
```

3. Update the `tauri-action` step to include signing configuration:

```yaml
- name: Build Tauri app
  uses: tauri-apps/tauri-action@v0
  env:
    GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
    TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
    TAURI_KEY_PASSWORD: ${{ secrets.TAURI_KEY_PASSWORD }}
  with:
    releaseId: ${{ needs.create-release.outputs.release_id }}
    args: "--config tauri.conf.json"
```

## Obtaining Signing Certificates

### macOS Certificates

1. Enroll in the Apple Developer Program
2. In Xcode, go to Preferences > Accounts > Manage Certificates
3. Create a "Developer ID Application" certificate
4. Export the certificate as a .p12 file with a password
5. Convert the .p12 file to base64:
   ```bash
   base64 -i certificate.p12 -o certificate-base64.txt
   ```
6. Add the content of certificate-base64.txt as the `APPLE_CERTIFICATE` secret

### Windows Certificates

1. Purchase a code signing certificate from a trusted Certificate Authority
2. Export the certificate as a .pfx file with a password
3. Convert the .pfx file to base64:
   ```bash
   base64 -i certificate.pfx -o certificate-base64.txt
   ```
4. Add the content of certificate-base64.txt as the `WINDOWS_CERTIFICATE` secret

## Using the CI/CD Pipeline

### Continuous Integration

The CI workflow runs automatically on:
- Every push to the `main` and `dev` branches
- Every pull request targeting the `main` and `dev` branches

This ensures that code changes build correctly and pass tests before being merged.

### Creating a Release

To create a production release:

1. **Option 1: Using Git Tags**
   - Update version numbers in:
     - `package.json`
     - `Cargo.toml`
     - `tauri.conf.json`
   - Commit the changes
   - Create and push a tag:
     ```bash
     git tag v1.0.0
     git push origin v1.0.0
     ```

2. **Option 2: Manual Trigger**
   - Go to the Actions tab in your GitHub repository
   - Select the "Release" workflow
   - Click "Run workflow"
   - Enter the version number (e.g., "1.0.0")
   - Click "Run workflow"

The release workflow will:
1. Create a draft release
2. Build the application for all platforms
3. Upload the builds as release assets
4. Publish the release

### Creating a Development Build

To create a development/preview build:

1. Go to the Actions tab in your GitHub repository
2. Select the "Development Build" workflow
3. Click "Run workflow"
4. Enter the branch name to build
5. Click "Run workflow"

The development build workflow will:
1. Build the application for all platforms
2. Create a pre-release with the builds as assets

## Versioning Strategy

Implexa follows Semantic Versioning (SemVer):
- **Major version** (x.0.0): Incompatible API changes
- **Minor version** (0.x.0): New features, backward compatible
- **Patch version** (0.0.x): Bug fixes, backward compatible

Version numbers should be maintained in:
1. `package.json` for the frontend
2. `Cargo.toml` for the Rust backend
3. `tauri.conf.json` for the Tauri configuration

## Troubleshooting

### Common Issues

1. **Build fails on Ubuntu**
   - Ensure all required system dependencies are installed
   - Check that the Ubuntu version in the workflow matches the one in the Tauri documentation

2. **Code signing fails**
   - Verify that all required secrets are correctly set
   - Check certificate expiration dates
   - Ensure the certificate has the correct usage rights

3. **Release assets not uploading**
   - Check that the `GITHUB_TOKEN` environment variable is correctly set
   - Verify that the release ID is correctly passed between jobs

### Viewing Workflow Logs

To diagnose issues:
1. Go to the Actions tab in your GitHub repository
2. Click on the failed workflow run
3. Expand the job that failed
4. Review the logs for error messages

## Additional Resources

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Tauri GitHub Action](https://github.com/tauri-apps/tauri-action)
- [Code Signing for Tauri](https://tauri.app/v1/guides/distribution/code-signing)
- [Semantic Versioning](https://semver.org/)