import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useNotifications } from './NotificationContext';

// Define revision status types
export type RevisionStatus = 'Draft' | 'InReview' | 'Released' | 'Obsolete';

// Define revision interface
export interface Revision {
  revision_id: number;
  part_id: number;
  version: string;
  status: RevisionStatus;
  created_by: string;
  commit_hash?: string;
}

// Define revision creation data
export interface RevisionCreationData {
  part_id: number;
  version: string;
  status: RevisionStatus;
  created_by: string;
  commit_hash?: string;
}

// Define revisions context type
interface RevisionContextType {
  loading: boolean;
  error: string | null;
  getRevision: (revisionId: number) => Promise<Revision | null>;
  getPartRevisions: (partId: number) => Promise<Revision[]>;
  getLatestRevision: (partId: number) => Promise<Revision | null>;
  getLatestReleasedRevision: (partId: number) => Promise<Revision | null>;
  createRevision: (data: RevisionCreationData) => Promise<Revision | null>;
  updateRevision: (revisionId: number, data: RevisionCreationData) => Promise<Revision | null>;
  updateRevisionStatus: (revisionId: number, newStatus: RevisionStatus) => Promise<Revision | null>;
  deleteRevision: (revisionId: number) => Promise<boolean>;
}

// Create the context
const RevisionContext = createContext<RevisionContextType | undefined>(undefined);

interface RevisionProviderProps {
  children: ReactNode;
}

/**
 * Provider component for revision data and operations
 */
export const RevisionProvider: React.FC<RevisionProviderProps> = ({ children }) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const { addNotification } = useNotifications();

  // Get a single revision by ID
  const getRevision = useCallback(async (revisionId: number): Promise<Revision | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Revision>('get_revision', { revision_id: revisionId });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch revision ${revisionId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching revision',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Get all revisions for a part
  const getPartRevisions = useCallback(async (partId: number): Promise<Revision[]> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Revision[]>('get_part_revisions', { part_id: partId });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch revisions for part ${partId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching part revisions',
      });
      return [];
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Get the latest revision for a part
  const getLatestRevision = useCallback(async (partId: number): Promise<Revision | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Revision>('get_latest_revision', { part_id: partId });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch latest revision for part ${partId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching latest revision',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Get the latest released revision for a part
  const getLatestReleasedRevision = useCallback(async (partId: number): Promise<Revision | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Revision>('get_latest_released_revision', { part_id: partId });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch latest released revision for part ${partId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching latest released revision',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Create a new revision
  const createRevision = useCallback(async (data: RevisionCreationData): Promise<Revision | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Revision>('create_revision', { revision_data: data });
      
      addNotification({
        type: 'success',
        message: `Revision ${data.version} created successfully`,
        title: 'Revision Created',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create revision';
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error creating revision',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Update an existing revision
  const updateRevision = useCallback(async (revisionId: number, data: RevisionCreationData): Promise<Revision | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Revision>('update_revision', { 
        revision_id: revisionId, 
        revision_data: data 
      });
      
      addNotification({
        type: 'success',
        message: `Revision ${data.version} updated successfully`,
        title: 'Revision Updated',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to update revision ${revisionId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error updating revision',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Update revision status
  const updateRevisionStatus = useCallback(async (revisionId: number, newStatus: RevisionStatus): Promise<Revision | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Revision>('update_revision_status', { 
        revision_id: revisionId, 
        status: newStatus 
      });
      
      addNotification({
        type: 'success',
        message: `Revision status changed to ${newStatus}`,
        title: 'Status Updated',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to update status for revision ${revisionId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error updating status',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Delete a revision
  const deleteRevision = useCallback(async (revisionId: number): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      await invoke<void>('delete_revision', { revision_id: revisionId });
      
      addNotification({
        type: 'success',
        message: 'Revision deleted successfully',
        title: 'Revision Deleted',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to delete revision ${revisionId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error deleting revision',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  const value = {
    loading,
    error,
    getRevision,
    getPartRevisions,
    getLatestRevision,
    getLatestReleasedRevision,
    createRevision,
    updateRevision,
    updateRevisionStatus,
    deleteRevision,
  };

  return <RevisionContext.Provider value={value}>{children}</RevisionContext.Provider>;
};

/**
 * Hook for accessing revision context
 */
export const useRevision = (): RevisionContextType => {
  const context = useContext(RevisionContext);
  if (context === undefined) {
    throw new Error('useRevision must be used within a RevisionProvider');
  }
  return context;
};