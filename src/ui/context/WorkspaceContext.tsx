import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useNotifications } from './NotificationContext';
import { Part } from './PartsContext';

// Define workspace interface
export interface Workspace {
  id: string;
  name: string;
  description: string;
  createdAt: string;
  updatedAt: string;
  parts: Part[];
}

// Define workspace creation data
export interface WorkspaceCreationData {
  name: string;
  description: string;
}

// Define workspace update data
export interface WorkspaceUpdateData {
  name?: string;
  description?: string;
}

// Define workspace context type
interface WorkspaceContextType {
  workspaces: Workspace[];
  currentWorkspace: Workspace | null;
  loading: boolean;
  error: string | null;
  fetchWorkspaces: () => Promise<void>;
  getWorkspace: (id: string) => Promise<Workspace | null>;
  createWorkspace: (data: WorkspaceCreationData) => Promise<Workspace | null>;
  updateWorkspace: (id: string, data: WorkspaceUpdateData) => Promise<Workspace | null>;
  deleteWorkspace: (id: string) => Promise<boolean>;
  switchWorkspace: (id: string) => Promise<boolean>;
  addPartToWorkspace: (workspaceId: string, partId: number) => Promise<boolean>;
  removePartFromWorkspace: (workspaceId: string, partId: number) => Promise<boolean>;
}

// Create the context
const WorkspaceContext = createContext<WorkspaceContextType | undefined>(undefined);

interface WorkspaceProviderProps {
  children: ReactNode;
}

/**
 * Provider component for workspace data and operations
 */
export const WorkspaceProvider: React.FC<WorkspaceProviderProps> = ({ children }) => {
  const [workspaces, setWorkspaces] = useState<Workspace[]>([]);
  const [currentWorkspace, setCurrentWorkspace] = useState<Workspace | null>(null);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const { addNotification } = useNotifications();

  // Fetch all workspaces
  const fetchWorkspaces = useCallback(async () => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Workspace[]>('get_workspaces');
      setWorkspaces(result);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to fetch workspaces';
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching workspaces',
      });
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Get a single workspace by ID
  const getWorkspace = useCallback(async (id: string): Promise<Workspace | null> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Workspace>('get_workspace', { workspaceId: id });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch workspace ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching workspace',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Create a new workspace
  const createWorkspace = useCallback(async (data: WorkspaceCreationData): Promise<Workspace | null> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Workspace>('create_workspace', { workspaceData: data });
      
      // Update the workspaces list with the new workspace
      setWorkspaces((prevWorkspaces) => [...prevWorkspaces, result]);
      
      addNotification({
        type: 'success',
        message: `Workspace "${result.name}" created successfully`,
        title: 'Workspace Created',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create workspace';
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error creating workspace',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Update an existing workspace
  const updateWorkspace = useCallback(async (id: string, data: WorkspaceUpdateData): Promise<Workspace | null> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Workspace>('update_workspace', { workspaceId: id, workspaceData: data });
      
      // Update the workspaces list with the updated workspace
      setWorkspaces((prevWorkspaces) =>
        prevWorkspaces.map((workspace) => (workspace.id === id ? result : workspace))
      );
      
      // If the current workspace is being updated, update it as well
      if (currentWorkspace && currentWorkspace.id === id) {
        setCurrentWorkspace(result);
      }
      
      addNotification({
        type: 'success',
        message: `Workspace "${result.name}" updated successfully`,
        title: 'Workspace Updated',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to update workspace ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error updating workspace',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification, currentWorkspace]);

  // Delete a workspace
  const deleteWorkspace = useCallback(async (id: string): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      await invoke<void>('delete_workspace', { workspaceId: id });
      
      // Remove the workspace from the workspaces list
      setWorkspaces((prevWorkspaces) => prevWorkspaces.filter((workspace) => workspace.id !== id));
      
      // If the current workspace is being deleted, set it to null
      if (currentWorkspace && currentWorkspace.id === id) {
        setCurrentWorkspace(null);
      }
      
      addNotification({
        type: 'success',
        message: 'Workspace deleted successfully',
        title: 'Workspace Deleted',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to delete workspace ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error deleting workspace',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification, currentWorkspace]);

  // Switch to a different workspace
  const switchWorkspace = useCallback(async (id: string): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Workspace>('get_workspace', { workspaceId: id });
      
      setCurrentWorkspace(result);
      
      addNotification({
        type: 'info',
        message: `Switched to workspace "${result.name}"`,
        title: 'Workspace Switched',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to switch to workspace ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error switching workspace',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Add a part to a workspace
  const addPartToWorkspace = useCallback(async (workspaceId: string, partId: number): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      await invoke<void>('add_part_to_workspace', { workspaceId, partId });
      
      // Refresh the workspace to get the updated parts list
      const updatedWorkspace = await invoke<Workspace>('get_workspace', { workspaceId });
      
      // Update the workspaces list with the updated workspace
      setWorkspaces((prevWorkspaces) =>
        prevWorkspaces.map((workspace) => (workspace.id === workspaceId ? updatedWorkspace : workspace))
      );
      
      // If the current workspace is being updated, update it as well
      if (currentWorkspace && currentWorkspace.id === workspaceId) {
        setCurrentWorkspace(updatedWorkspace);
      }
      
      addNotification({
        type: 'success',
        message: 'Part added to workspace successfully',
        title: 'Part Added',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to add part ${partId} to workspace ${workspaceId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error adding part',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification, currentWorkspace]);

  // Remove a part from a workspace
  const removePartFromWorkspace = useCallback(async (workspaceId: string, partId: number): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      await invoke<void>('remove_part_from_workspace', { workspaceId, partId });
      
      // Refresh the workspace to get the updated parts list
      const updatedWorkspace = await invoke<Workspace>('get_workspace', { workspaceId });
      
      // Update the workspaces list with the updated workspace
      setWorkspaces((prevWorkspaces) =>
        prevWorkspaces.map((workspace) => (workspace.id === workspaceId ? updatedWorkspace : workspace))
      );
      
      // If the current workspace is being updated, update it as well
      if (currentWorkspace && currentWorkspace.id === workspaceId) {
        setCurrentWorkspace(updatedWorkspace);
      }
      
      addNotification({
        type: 'success',
        message: 'Part removed from workspace successfully',
        title: 'Part Removed',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to remove part ${partId} from workspace ${workspaceId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error removing part',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification, currentWorkspace]);

  const value = {
    workspaces,
    currentWorkspace,
    loading,
    error,
    fetchWorkspaces,
    getWorkspace,
    createWorkspace,
    updateWorkspace,
    deleteWorkspace,
    switchWorkspace,
    addPartToWorkspace,
    removePartFromWorkspace,
  };

  return <WorkspaceContext.Provider value={value}>{children}</WorkspaceContext.Provider>;
};

/**
 * Hook for accessing workspace context
 */
export const useWorkspace = (): WorkspaceContextType => {
  const context = useContext(WorkspaceContext);
  if (context === undefined) {
    throw new Error('useWorkspace must be used within a WorkspaceProvider');
  }
  return context;
};