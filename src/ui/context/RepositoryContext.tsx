import React, { createContext, useContext, useState, useCallback } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { RepositoryService, Repository } from '../services/RepositoryService';

interface RepositoryContextType {
  repository: Repository | null;
  isLoading: boolean;
  error: string | null;
  createRepository: (path: string, name: string) => Promise<void>;
  openRepository: (path?: string) => Promise<void>;
  closeRepository: () => Promise<void>;
  refreshRepository: () => Promise<void>;
  selectDirectoryTemplate: (templateType: string) => Promise<void>;
  selectedTemplate: string;
}

const RepositoryContext = createContext<RepositoryContextType | undefined>(undefined);

export const useRepository = () => {
  const context = useContext(RepositoryContext);
  if (context === undefined) {
    throw new Error('useRepository must be used within a RepositoryProvider');
  }
  return context;
};

export const RepositoryProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [repository, setRepository] = useState<Repository | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [selectedTemplate, setSelectedTemplate] = useState<string>('standard');

  const refreshRepository = useCallback(async () => {
    if (!repository) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      const repoInfo = await RepositoryService.getRepositoryInfo(repository.path);
      setRepository(repoInfo);
    } catch (err) {
      setError(`Failed to refresh repository: ${err}`);
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  }, [repository]);

  const createRepository = useCallback(async (path: string, name: string) => {
    setIsLoading(true);
    setError(null);
    
    try {
      // Create the repository directory if it doesn't exist
      const repoPath = `${path}/${name}`;
      
      // Initialize the repository
      const repoInfo = await RepositoryService.createRepository(repoPath, selectedTemplate);
      
      setRepository(repoInfo);
    } catch (err) {
      setError(`Failed to create repository: ${err}`);
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  }, [selectedTemplate]);

  const openRepository = useCallback(async (path?: string) => {
    setIsLoading(true);
    setError(null);
    
    try {
      // If no path is provided, open a directory selection dialog
      const selectedPath = path || await open({
        directory: true,
        multiple: false,
        title: 'Select Repository Directory'
      });
      
      // User cancelled the dialog
      if (!selectedPath) {
        setIsLoading(false);
        return;
      }
      
      // Open the repository
      const repoPath = typeof selectedPath === 'string' ? selectedPath : selectedPath[0];
      const repoInfo = await RepositoryService.openRepository(repoPath);
      
      setRepository(repoInfo);
    } catch (err) {
      setError(`Failed to open repository: ${err}`);
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  }, []);

  const closeRepository = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      if (repository) {
        await RepositoryService.closeRepository(repository.path);
      }
      
      setRepository(null);
    } catch (err) {
      setError(`Failed to close repository: ${err}`);
      console.error(err);
    } finally {
      setIsLoading(false);
    }
  }, [repository]);

  const selectDirectoryTemplate = useCallback(async (templateType: string) => {
    setSelectedTemplate(templateType);
  }, []);

  return (
    <RepositoryContext.Provider
      value={{
        repository,
        isLoading,
        error,
        createRepository,
        openRepository,
        closeRepository,
        refreshRepository,
        selectDirectoryTemplate,
        selectedTemplate
      }}
    >
      {children}
    </RepositoryContext.Provider>
  );
};