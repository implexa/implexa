import { invoke } from '@tauri-apps/api/tauri';

export interface Repository {
  id: string;
  path: string;
  name: string;
  currentBranch: string;
  hasChanges: boolean;
  lfsEnabled: boolean;
}

/**
 * Service for interacting with Git repositories
 */
export class RepositoryService {
  /**
   * Create a new repository with the specified template
   * @param path Path where the repository will be created
   * @param templateType Type of directory template to use (minimal, standard, extended)
   * @returns Repository information
   */
  static async createRepository(path: string, templateType: string): Promise<Repository> {
    try {
      return await invoke<Repository>('create_repository', {
        path,
        templateType
      });
    } catch (error) {
      console.error('Error creating repository:', error);
      throw error;
    }
  }

  /**
   * Open an existing repository
   * @param path Path to the repository
   * @returns Repository information
   */
  static async openRepository(path: string): Promise<Repository> {
    try {
      return await invoke<Repository>('open_repository', {
        path
      });
    } catch (error) {
      console.error('Error opening repository:', error);
      throw error;
    }
  }

  /**
   * Close the current repository
   * @param path Path to the repository
   */
  static async closeRepository(path: string): Promise<void> {
    try {
      await invoke('close_repository', {
        path
      });
    } catch (error) {
      console.error('Error closing repository:', error);
      throw error;
    }
  }

  /**
   * Get information about a repository
   * @param path Path to the repository
   * @returns Repository information
   */
  static async getRepositoryInfo(path: string): Promise<Repository> {
    try {
      return await invoke<Repository>('get_repository_info', {
        path
      });
    } catch (error) {
      console.error('Error getting repository info:', error);
      throw error;
    }
  }
}