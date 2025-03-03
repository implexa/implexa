//! Auth Provider
//!
//! This module handles Git authentication and credentials. It provides functionality
//! for managing Git authentication and credentials, supporting multiple authentication methods
//! (SSH, HTTPS, tokens), and securely storing and retrieving credentials.

use std::path::{Path, PathBuf};
use git2::{Cred, CredentialType};
use crate::git_backend::{GitBackendError, Result, Credentials, AuthConfig};

/// Auth Provider
pub struct AuthProvider {
    /// The authentication configuration
    config: AuthConfig,
    /// Stored credentials
    credentials: std::collections::HashMap<String, Credentials>,
}

impl AuthProvider {
    /// Creates a new AuthProvider with the specified configuration
    pub fn new(config: AuthConfig) -> Result<Self> {
        Ok(Self {
            config,
            credentials: std::collections::HashMap::new(),
        })
    }
    
    /// Gets the credentials for the specified URL
    pub fn get_credentials(&self, url: &str) -> Result<Credentials> {
        // Check if we have stored credentials for this URL
        if let Some(creds) = self.credentials.get(url) {
            return Ok(creds.clone());
        }
        
        // Check if we should use the system credential helper
        if self.config.use_credential_helper {
            // This would use the system credential helper to get credentials
            // For now, we'll return an error
            return Err(GitBackendError::AuthError(format!("No credentials found for URL: {}", url)));
        }
        
        // Check if we should use SSH
        if url.starts_with("git@") || url.starts_with("ssh://") {
            // Check if we have an SSH key path
            if let Some(ssh_key_path) = &self.config.ssh_key_path {
                let public_key = ssh_key_path.with_extension("pub");
                
                return Ok(Credentials::SshKey {
                    username: "git".to_string(),
                    public_key,
                    private_key: ssh_key_path.clone(),
                    passphrase: None,
                });
            }
            
            // Check if we should use the SSH agent
            if self.config.use_ssh_agent {
                return Ok(Credentials::SshKey {
                    username: "git".to_string(),
                    public_key: PathBuf::new(),
                    private_key: PathBuf::new(),
                    passphrase: None,
                });
            }
        }
        
        // No credentials found
        Err(GitBackendError::AuthError(format!("No credentials found for URL: {}", url)))
    }
    
    /// Sets the credentials for the specified URL
    pub fn set_credentials(&self, url: &str, credentials: Credentials) -> Result<()> {
        // In a real implementation, we would store the credentials securely
        // For now, we'll just store them in memory
        let mut credentials_map = self.credentials.clone();
        credentials_map.insert(url.to_string(), credentials);
        
        Ok(())
    }
    
    /// Clears the credentials for the specified URL
    pub fn clear_credentials(&self, url: &str) -> Result<()> {
        // In a real implementation, we would remove the credentials from secure storage
        // For now, we'll just remove them from memory
        let mut credentials_map = self.credentials.clone();
        credentials_map.remove(url);
        
        Ok(())
    }
    
    /// Gets the git2 credentials for the specified URL
    pub(crate) fn get_git2_credentials(
        &self,
        url: &str,
        username_from_url: Option<&str>,
        allowed_types: CredentialType,
    ) -> Result<Cred> {
        // Try to get the credentials for this URL
        let credentials = match self.get_credentials(url) {
            Ok(creds) => creds,
            Err(_) => {
                // If we don't have credentials, try to use the default authentication methods
                
                // Check if we can use SSH key authentication
                if allowed_types.contains(CredentialType::SSH_KEY) && self.config.ssh_key_path.is_some() {
                    let ssh_key_path = self.config.ssh_key_path.as_ref().unwrap();
                    let public_key = ssh_key_path.with_extension("pub");
                    
                    return Ok(Cred::ssh_key(
                        username_from_url.unwrap_or("git"),
                        Some(&public_key),
                        ssh_key_path,
                        None,
                    ).map_err(|e| GitBackendError::AuthError(format!("Failed to create SSH key credentials: {}", e)))?);
                }
                
                // Check if we can use SSH agent authentication
                if allowed_types.contains(CredentialType::SSH_KEY) && self.config.use_ssh_agent {
                    return Ok(Cred::ssh_key_from_agent(
                        username_from_url.unwrap_or("git"),
                    ).map_err(|e| GitBackendError::AuthError(format!("Failed to create SSH agent credentials: {}", e)))?);
                }
                
                // Check if we can use default credentials
                if allowed_types.contains(CredentialType::DEFAULT) {
                    return Ok(Cred::default()
                        .map_err(|e| GitBackendError::AuthError(format!("Failed to create default credentials: {}", e)))?);
                }
                
                // No credentials available
                return Err(GitBackendError::AuthError(format!("No credentials available for URL: {}", url)));
            }
        };
        
        // Convert the credentials to git2 credentials
        match credentials {
            Credentials::UserPass { username, password } => {
                if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
                    Ok(Cred::userpass_plaintext(&username, &password)
                        .map_err(|e| GitBackendError::AuthError(format!("Failed to create userpass credentials: {}", e)))?)
                } else {
                    Err(GitBackendError::AuthError("Userpass authentication not allowed".to_string()))
                }
            },
            Credentials::SshKey { username, public_key, private_key, passphrase } => {
                if allowed_types.contains(CredentialType::SSH_KEY) {
                    Ok(Cred::ssh_key(
                        &username,
                        if public_key.as_os_str().is_empty() { None } else { Some(&public_key) },
                        &private_key,
                        passphrase.as_deref(),
                    ).map_err(|e| GitBackendError::AuthError(format!("Failed to create SSH key credentials: {}", e)))?)
                } else {
                    Err(GitBackendError::AuthError("SSH key authentication not allowed".to_string()))
                }
            },
            Credentials::Token { username, token } => {
                if allowed_types.contains(CredentialType::USER_PASS_PLAINTEXT) {
                    Ok(Cred::userpass_plaintext(&username, &token)
                        .map_err(|e| GitBackendError::AuthError(format!("Failed to create token credentials: {}", e)))?)
                } else {
                    Err(GitBackendError::AuthError("Token authentication not allowed".to_string()))
                }
            },
        }
    }
}