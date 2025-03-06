import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useNotifications } from './NotificationContext';

// Define relationship type
export type RelationshipType = 'Assembly' | 'Reference' | 'Alternate';

// Define relationship interface
export interface Relationship {
  relationship_id: number;
  parent_id: number;
  child_id: number;
  relationship_type: RelationshipType;
  quantity: number;
  unit?: string;
  description?: string;
}

// Define relationship creation data
export interface RelationshipCreationData {
  parent_id: number;
  child_id: number;
  relationship_type: RelationshipType;
  quantity: number;
  unit?: string;
  description?: string;
}

// Define relationships context type
interface RelationshipsContextType {
  loading: boolean;
  error: string | null;
  getRelationship: (relationshipId: number) => Promise<Relationship | null>;
  getParentRelationships: (childId: number) => Promise<Relationship[]>;
  getChildRelationships: (parentId: number) => Promise<Relationship[]>;
  createRelationship: (data: RelationshipCreationData) => Promise<Relationship | null>;
  updateRelationship: (relationshipId: number, data: RelationshipCreationData) => Promise<Relationship | null>;
  deleteRelationship: (relationshipId: number) => Promise<boolean>;
}

// Create the context
const RelationshipsContext = createContext<RelationshipsContextType | undefined>(undefined);

interface RelationshipsProviderProps {
  children: ReactNode;
}

/**
 * Provider component for relationships data and operations
 */
export const RelationshipsProvider: React.FC<RelationshipsProviderProps> = ({ children }) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const { addNotification } = useNotifications();

  // Get a single relationship by ID
  const getRelationship = useCallback(async (relationshipId: number): Promise<Relationship | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Relationship>('get_relationship', { relationship_id: relationshipId });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch relationship ${relationshipId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching relationship',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Get all parent relationships for a part
  const getParentRelationships = useCallback(async (childId: number): Promise<Relationship[]> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Relationship[]>('get_parent_relationships', { child_id: childId });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch parent relationships for part ${childId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching parent relationships',
      });
      return [];
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Get all child relationships for a part
  const getChildRelationships = useCallback(async (parentId: number): Promise<Relationship[]> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Relationship[]>('get_child_relationships', { parent_id: parentId });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch child relationships for part ${parentId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching child relationships',
      });
      return [];
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Create a new relationship
  const createRelationship = useCallback(async (data: RelationshipCreationData): Promise<Relationship | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Relationship>('create_relationship', { relationship_data: data });
      
      addNotification({
        type: 'success',
        message: 'Relationship created successfully',
        title: 'Relationship Created',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create relationship';
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error creating relationship',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Update an existing relationship
  const updateRelationship = useCallback(async (relationshipId: number, data: RelationshipCreationData): Promise<Relationship | null> => {
    setLoading(true);
    setError(null);
    
    try {
      const result = await invoke<Relationship>('update_relationship', { 
        relationship_id: relationshipId, 
        relationship_data: data 
      });
      
      addNotification({
        type: 'success',
        message: 'Relationship updated successfully',
        title: 'Relationship Updated',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to update relationship ${relationshipId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error updating relationship',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Delete a relationship
  const deleteRelationship = useCallback(async (relationshipId: number): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      await invoke<void>('delete_relationship', { relationship_id: relationshipId });
      
      addNotification({
        type: 'success',
        message: 'Relationship deleted successfully',
        title: 'Relationship Deleted',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to delete relationship ${relationshipId}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error deleting relationship',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  const value = {
    loading,
    error,
    getRelationship,
    getParentRelationships,
    getChildRelationships,
    createRelationship,
    updateRelationship,
    deleteRelationship,
  };

  return <RelationshipsContext.Provider value={value}>{children}</RelationshipsContext.Provider>;
};

/**
 * Hook for accessing relationships context
 */
export const useRelationships = (): RelationshipsContextType => {
  const context = useContext(RelationshipsContext);
  if (context === undefined) {
    throw new Error('useRelationships must be used within a RelationshipsProvider');
  }
  return context;
};