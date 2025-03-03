# Implexa Development Environment Setup

This document provides instructions for setting up the development environment for Implexa, a hardware-focused PLM/PDM solution built with Tauri and Rust.

## Prerequisites

The Implexa development environment requires the following tools:

- **Rust**: For backend development
- **Node.js**: For frontend development
- **Git with Git-LFS**: For version control and large file handling
- **Visual Studio Code** (recommended): For development
- **SQLite**: For database operations
- **Tauri CLI**: For building and running the application

## Windows Setup

### 1. Install Rust

1. Download and run the Rust installer from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Follow the installer prompts to install Rust and Cargo
3. Open a new Command Prompt and verify the installation:
   ```cmd
   rustc --version
   cargo --version
   ```

### 2. Install Node.js

1. Download the LTS version of Node.js from [https://nodejs.org/](https://nodejs.org/)
2. Run the installer and follow the prompts
3. Verify the installation:
   ```cmd
   node --version
   npm --version
   ```

### 3. Install Git with Git-LFS

1. Download Git for Windows from [https://git-scm.com/download/win](https://git-scm.com/download/win)
2. During installation:
   - Choose to use Git from the command line and from 3rd-party software
   - Select "Checkout as-is, commit as-is" for line ending conversions
   - Enable Git Credential Manager
3. Download Git-LFS from [https://git-lfs.github.com/](https://git-lfs.github.com/)
4. Run the Git-LFS installer
5. Open a new Command Prompt and verify the installations:
   ```cmd
   git --version
   git lfs --version
   ```
6. Configure Git-LFS:
   ```cmd
   git lfs install
   ```

### 4. Install Visual Studio Code (Recommended)

1. Download VS Code from [https://code.visualstudio.com/](https://code.visualstudio.com/)
2. Run the installer and follow the prompts
3. Recommended extensions for Implexa development:
   - Rust Analyzer
   - Tauri
   - SQLite
   - GitLens
   - ESLint
   - Prettier

### 5. Install SQLite

1. Download the SQLite tools for Windows from [https://www.sqlite.org/download.html](https://www.sqlite.org/download.html)
   - Download the "Precompiled Binaries for Windows" bundle
2. Extract the files to a directory (e.g., `C:\sqlite`)
3. Add the directory to your PATH:
   - Open the Start menu and search for "Environment Variables"
   - Click "Edit the system environment variables"
   - Click the "Environment Variables" button
   - Under "System variables", find the "Path" variable and click "Edit"
   - Click "New" and add the path to your SQLite directory (e.g., `C:\sqlite`)
   - Click "OK" on all dialogs
4. Verify the installation:
   ```cmd
   sqlite3 --version
   ```

### 6. Install C++ Build Tools (Required for Tauri)

1. Download the Visual Studio Build Tools from [https://visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Run the installer
3. Select "Desktop development with C++" workload
4. Complete the installation

### 7. Install WebView2 Runtime (Required for Tauri)

1. Download the WebView2 Runtime from [https://developer.microsoft.com/en-us/microsoft-edge/webview2/](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
2. Run the installer and follow the prompts

### 8. Clone the Repository and Set Up the Project

1. Clone the Implexa repository:
   ```cmd
   git clone https://github.com/your-organization/implexa.git
   cd implexa
   ```
2. Install Node.js dependencies:
   ```cmd
   npm install
   ```
3. Install Tauri CLI:
   ```cmd
   cargo install tauri-cli
   ```

### 9. Run the Development Server

1. Start the development server:
   ```cmd
   npm run tauri dev
   ```
   This will start both the frontend development server and the Tauri application.

## macOS Setup (Outline)

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install Node.js
```bash
# Using Homebrew
brew install node

# Or using NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
nvm install --lts
```

### 3. Install Git with Git-LFS
```bash
# Using Homebrew
brew install git git-lfs
git lfs install
```

### 4. Install Xcode Command Line Tools
```bash
xcode-select --install
```

### 5. Install SQLite
```bash
brew install sqlite
```

### 6. Clone and Set Up the Project
```bash
git clone https://github.com/your-organization/implexa.git
cd implexa
npm install
cargo install tauri-cli
```

### 7. Run the Development Server
```bash
npm run tauri dev
```

## Linux Setup (Outline)

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install Node.js
```bash
# Using package manager (Ubuntu/Debian)
curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
sudo apt-get install -y nodejs

# Or using NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh | bash
nvm install --lts
```

### 3. Install Git with Git-LFS
```bash
# Ubuntu/Debian
sudo apt-get install git
curl -s https://packagecloud.io/install/repositories/github/git-lfs/script.deb.sh | sudo bash
sudo apt-get install git-lfs
git lfs install
```

### 4. Install System Dependencies for Tauri
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### 5. Install SQLite
```bash
sudo apt-get install sqlite3 libsqlite3-dev
```

### 6. Clone and Set Up the Project
```bash
git clone https://github.com/your-organization/implexa.git
cd implexa
npm install
cargo install tauri-cli
```

### 7. Run the Development Server
```bash
npm run tauri dev
```

## Troubleshooting

### Common Issues on Windows

1. **Rust build fails with linker errors**
   - Make sure you have the C++ Build Tools installed correctly
   - Try running `rustup update` to ensure Rust is up to date

2. **Tauri fails to start with WebView2 errors**
   - Ensure WebView2 Runtime is installed correctly
   - Try reinstalling the WebView2 Runtime

3. **Git-LFS issues**
   - Run `git lfs install --skip-repo` to ensure Git-LFS is configured globally
   - For repository-specific issues, try `git lfs fetch --all` and `git lfs pull`

### Common Issues on macOS

1. **Xcode Command Line Tools not found**
   - Run `xcode-select --install` to install the tools
   - If issues persist, try reinstalling with `sudo rm -rf /Library/Developer/CommandLineTools && xcode-select --install`

### Common Issues on Linux

1. **Missing Tauri dependencies**
   - Ensure all system dependencies are installed for your specific distribution
   - For distribution-specific instructions, refer to the [Tauri prerequisites guide](https://tauri.app/v1/guides/getting-started/prerequisites)

## Additional Resources

- [Rust Documentation](https://www.rust-lang.org/learn)
- [Tauri Documentation](https://tauri.app/v1/guides/)
- [React Documentation](https://reactjs.org/docs/getting-started.html)
- [Git-LFS Documentation](https://git-lfs.github.com/)
- [SQLite Documentation](https://www.sqlite.org/docs.html)