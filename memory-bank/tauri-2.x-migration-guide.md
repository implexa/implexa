# Implexa: Tauri 1.5.4 to 2.3.1 Migration Guide

This guide outlines the steps required to migrate the Implexa application from Tauri 1.5.4 to Tauri 2.3.1. The migration will modernize the application's architecture while enabling new features like mobile support and an improved plugin system.

## Table of Contents

1. [Pre-Migration Preparations](#pre-migration-preparations)
2. [Update Dependencies](#update-dependencies)
3. [Configuration Changes](#configuration-changes)
4. [Backend Command Changes](#backend-command-changes)
5. [Frontend API Changes](#frontend-api-changes)
6. [Code Examples](#code-examples)
7. [Testing the Migration](#testing-the-migration)
8. [Additional Resources](#additional-resources)

## Pre-Migration Preparations

Before starting the migration, complete the following preparatory steps:

1. **Create a new branch**:
   ```bash
   git checkout -b feature/tauri-2-migration
   ```

2. **Ensure all tests pass**:
   ```bash
   cargo test
   npm test
   ```

3. **Document current application state**: Make note of all working Tauri features to verify after migration.

4. **Backup your project**: Ensure you have a full backup before proceeding.

## Update Dependencies

### 1. Update Tauri CLI

```bash
cargo install tauri-cli
```

### 2. Update `Cargo.toml`

Replace the current Tauri dependency with the new version:

```toml
# Old dependency
tauri = { version = "1.5.4", features = [ "dialog-all", "path-all", "shell-open", "fs-all", "window-all"] }

# New dependency
tauri = { version = "2.3.1", features = [] }
```

Update build dependencies:

```toml
# Old build dependency
tauri-build = { version = "1.5.4", features = [] }

# New build dependency
tauri-build = { version = "2.3.1", features = [] }
```

### 3. Update `package.json`

Update JavaScript dependencies:

```json
{
  "dependencies": {
    "@tauri-apps/api": "^2.3.1"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2.3.1"
  }
}
```

Then run:

```bash
npm install
```

## Configuration Changes

### 1. Update `tauri.conf.json`

Tauri 2.x uses a different configuration structure. Create a new `tauri.conf.json` based on the current one:

```json
{
  "$schema": "https://tauri.app/v2/api/config/#schema",
  "identifier": "com.implexa.dev",
  "productName": "Implexa",
  "version": "0.1.0",
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "../dist"
  },
  "app": {
    "security": {
      "csp": null
    },
    "windows": [
      {
        "fullscreen": false,
        "height": 800,
        "resizable": true,
        "title": "Implexa - Git-Based PLM/PDM Solution",
        "width": 1200,
        "fileDropEnabled": false
      }
    ]
  },
  "bundle": {
    "active": true,
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "targets": ["deb", "msi", "dmg", "updater"]
  },
  "plugins": {
    "shell": {
      "open": true
    },
    "dialog": {
      "all": true
    },
    "fs": {
      "all": true,
      "scope": {
        "allow": ["**"]
      }
    },
    "path": {
      "all": true
    },
    "window": {
      "all": true
    }
  },
  "updater": {
    "active": false
  }
}
```

Key changes:
- `tauri.allowlist` is now moved to the `plugins` section
- `tauri.bundle.identifier` is now at the root level as `identifier`
- `build.devPath` is now `build.devUrl`
- `build.distDir` is now `build.frontendDist`

### 2. Update `build.rs`

Update your build script to use Tauri 2.x's build approach:

```rust
fn main() {
    tauri_build::build();
}
```

## Backend Command Changes

### 1. Update `src/commands.rs`

Tauri 2.x uses a different command system. Here's how to update your commands:

```rust
use tauri::State;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::sync::Mutex;
use crate::{GitBackendManager, GitBackendConfig, AuthConfig, RepositoryInfo};

// Keep the RepositoryDto struct as is

// Keep the GitBackendState struct as is

// Update command definition
#[tauri::command]
pub async fn create_repository(
    path: String,
    template_type: String,
    git_state: State<'_, GitBackendState>,
) -> Result<RepositoryDto, String> {
    // Implementation remains the same
}

#[tauri::command]
pub async fn open_repository(
    path: String,
    git_state: State<'_, GitBackendState>,
) -> Result<RepositoryDto, String> {
    // Implementation remains the same
}

#[tauri::command]
pub async fn close_repository(
    path: String,
    _git_state: State<'_, GitBackendState>,
) -> Result<(), String> {
    // Implementation remains the same
}

#[tauri::command]
pub async fn get_repository_info(
    path: String,
    git_state: State<'_, GitBackendState>,
) -> Result<RepositoryDto, String> {
    // Implementation remains the same
}

// Keep the init_git_backend function as is
```

The main change is that we no longer need the `#[command]` macro, but rather use `#[tauri::command]` which is the same as before.

### 2. Update `src/main.rs`

Update the main.rs file to use the new plugin system and command registration:

```rust
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, State};
use std::sync::Mutex;
use implexa::commands::*;

// Define the same AppState struct

// Define the same increment_counter command

// Define the same get_counter command

fn main() {
    // Initialize logging
    env_logger::init();

    // Build the Tauri application
    tauri::Builder::default()
        .setup(|app| {
            // Initialize the application state
            app.manage(AppState {
                counter: Mutex::new(0),
            });
            
            // Initialize git backend state
            app.manage(init_git_backend());
            
            // Log that the application has started
            log::info!("Implexa application started");
            
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_path::init())
        .plugin(tauri_plugin_window::init())
        .invoke_handler(tauri::generate_handler![
            increment_counter,
            get_counter,
            create_repository,
            open_repository,
            close_repository,
            get_repository_info
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Implexa application");
}
```

Key changes:
- Add explicit plugin initialization
- Register all commands in the invoke_handler

## Frontend API Changes

### 1. Update `src/main.tsx`

The frontend API has changed significantly in Tauri 2.x. Update your imports and function calls:

```tsx
import React from 'react';
import ReactDOM from 'react-dom/client';
import { invoke } from '@tauri-apps/api';
import App from './ui/App';
import './styles.css';

// Initialize Tauri commands
async function initTauri() {
  try {
    // Initialize the Git backend
    await invoke('init_git_backend');
    console.log('Git backend initialized');
  } catch (error) {
    console.error('Failed to initialize Git backend:', error);
  }
}

// Initialize Tauri
initTauri().catch(console.error);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

### 2. Update `src/ui/context/PartsContext.tsx`

Update the Tauri API imports and function calls:

```tsx
// Old import
import { invoke } from '@tauri-apps/api/tauri';

// New import
import { invoke } from '@tauri-apps/api';
```

All invoke calls should work the same way.

### 3. Other Components

Similarly, update any other components that use Tauri APIs:

```tsx
// Old imports
import { open } from '@tauri-apps/api/dialog';
import { readTextFile } from '@tauri-apps/api/fs';

// New imports
import { dialog } from '@tauri-apps/api';
import { fs } from '@tauri-apps/api';

// Old usage
const result = await open({ multiple: false });
const content = await readTextFile(result as string);

// New usage
const result = await dialog.open({ multiple: false });
const content = await fs.readTextFile(result as string);
```

## Code Examples

### Example 1: Updated `commands.rs` Registration in `main.rs`

```rust
fn main() {
    env_logger::init();

    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState { counter: Mutex::new(0) });
            app.manage(init_git_backend());
            log::info!("Implexa application started");
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_path::init())
        .plugin(tauri_plugin_window::init())
        .invoke_handler(tauri::generate_handler![
            increment_counter,
            get_counter,
            create_repository,
            open_repository,
            close_repository,
            get_repository_info
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Implexa application");
}
```

### Example 2: Updated Frontend File Operation

```typescript
import { fs, path } from '@tauri-apps/api';

async function readFile() {
  try {
    const filePath = await path.resolveResource('config.json');
    const content = await fs.readTextFile(filePath);
    console.log('File content:', content);
  } catch (error) {
    console.error('Error reading file:', error);
  }
}
```

### Example 3: Updated Frontend Dialog

```typescript
import { dialog } from '@tauri-apps/api';

async function openFile() {
  try {
    const selected = await dialog.open({
      multiple: false,
      filters: [{
        name: 'CSV Files',
        extensions: ['csv']
      }]
    });
    
    if (selected) {
      console.log('Selected file:', selected);
    }
  } catch (error) {
    console.error('Error opening file dialog:', error);
  }
}
```

## Testing the Migration

After completing the migration steps, test the application thoroughly:

1. **Development Mode Test**:
   ```bash
   cargo tauri dev
   ```

2. **Build Test**:
   ```bash
   cargo tauri build
   ```

3. **Functional Testing Checklist**:
   - [ ] Application starts without errors
   - [ ] Repository creation works
   - [ ] Repository opening works
   - [ ] Part management features work
   - [ ] File system operations work
   - [ ] Dialogs work
   - [ ] Window management works

## Additional Resources

- [Official Tauri 2.x Migration Guide](https://tauri.app/v2/guides/migrate-from-v1/)
- [Tauri 2.3.1 API Documentation](https://tauri.app/v2/api/)
- [Tauri 2.x Plugin System](https://tauri.app/v2/guides/plugins/official/)
- [Tauri GitHub Repository](https://github.com/tauri-apps/tauri)
- [Tauri Discord Community](https://discord.gg/tauri)

## Conclusion

Migrating from Tauri 1.5.4 to Tauri 2.3.1 requires substantial changes to your application's configuration and code, but offers significant benefits including better mobile support, improved plugin system, and future-proofing your application. 

By following this guide, you should be able to successfully migrate the Implexa application to the latest version of Tauri while maintaining all existing functionality.
