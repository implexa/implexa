import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { useNotifications } from './NotificationContext';

// Define part status types
export type PartStatus = 'Draft' | 'In Review' | 'Released' | 'Obsolete';

// Define part interface
export interface Part {
  id: number;
  displayId: string; // Format: [Category]-[Subcategory]-[Number]
  name: string;
  description: string;
  status: PartStatus;
  category: string;
  subcategory: string;
  createdAt: string;
  updatedAt: string;
  properties: Record<string, string>;
}

// Define part creation data
export interface PartCreationData {
  name: string;
  description: string;
  category: string;
  subcategory: string;
  properties?: Record<string, string>;
}

// Define part update data
export interface PartUpdateData {
  name?: string;
  description?: string;
  properties?: Record<string, string>;
}

// Define parts context type
interface PartsContextType {
  parts: Part[];
  loading: boolean;
  error: string | null;
  fetchParts: () => Promise<void>;
  getPart: (id: number) => Promise<Part | null>;
  createPart: (data: PartCreationData) => Promise<Part | null>;
  updatePart: (id: number, data: PartUpdateData) => Promise<Part | null>;
  changePartStatus: (id: number, newStatus: PartStatus) => Promise<boolean>;
  deletePart: (id: number) => Promise<boolean>;
}

// Create the context
const PartsContext = createContext<PartsContextType | undefined>(undefined);

interface PartsProviderProps {
  children: ReactNode;
}

/**
 * Provider component for parts data and operations
 */
export const PartsProvider: React.FC<PartsProviderProps> = ({ children }) => {
  const [parts, setParts] = useState<Part[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const { addNotification } = useNotifications();

  // Fetch all parts
  const fetchParts = useCallback(async () => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Part[]>('get_parts');
      setParts(result);
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to fetch parts';
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching parts',
      });
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Get a single part by ID
  const getPart = useCallback(async (id: number): Promise<Part | null> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Part>('get_part', { partId: id });
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to fetch part ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error fetching part',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Create a new part
  const createPart = useCallback(async (data: PartCreationData): Promise<Part | null> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Part>('create_part', { partData: data });
      
      // Update the parts list with the new part
      setParts((prevParts) => [...prevParts, result]);
      
      addNotification({
        type: 'success',
        message: `Part ${result.displayId} created successfully`,
        title: 'Part Created',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to create part';
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error creating part',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Update an existing part
  const updatePart = useCallback(async (id: number, data: PartUpdateData): Promise<Part | null> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      const result = await invoke<Part>('update_part', { partId: id, partData: data });
      
      // Update the parts list with the updated part
      setParts((prevParts) =>
        prevParts.map((part) => (part.id === id ? result : part))
      );
      
      addNotification({
        type: 'success',
        message: `Part ${result.displayId} updated successfully`,
        title: 'Part Updated',
      });
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to update part ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error updating part',
      });
      return null;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Change part status
  const changePartStatus = useCallback(async (id: number, newStatus: PartStatus): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      await invoke<void>('change_part_status', { partId: id, status: newStatus });
      
      // Update the parts list with the new status
      setParts((prevParts) =>
        prevParts.map((part) =>
          part.id === id ? { ...part, status: newStatus } : part
        )
      );
      
      addNotification({
        type: 'success',
        message: `Part status changed to ${newStatus}`,
        title: 'Status Updated',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to change part status for ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error changing status',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  // Delete a part
  const deletePart = useCallback(async (id: number): Promise<boolean> => {
    setLoading(true);
    setError(null);
    
    try {
      // In a real implementation, this would call the Tauri backend
      await invoke<void>('delete_part', { partId: id });
      
      // Remove the part from the parts list
      setParts((prevParts) => prevParts.filter((part) => part.id !== id));
      
      addNotification({
        type: 'success',
        message: 'Part deleted successfully',
        title: 'Part Deleted',
      });
      
      return true;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : `Failed to delete part ${id}`;
      setError(errorMessage);
      addNotification({
        type: 'error',
        message: errorMessage,
        title: 'Error deleting part',
      });
      return false;
    } finally {
      setLoading(false);
    }
  }, [addNotification]);

  const value = {
    parts,
    loading,
    error,
    fetchParts,
    getPart,
    createPart,
    updatePart,
    changePartStatus,
    deletePart,
  };

  return <PartsContext.Provider value={value}>{children}</PartsContext.Provider>;
};

/**
 * Hook for accessing parts context
 */
export const useParts = (): PartsContextType => {
  const context = useContext(PartsContext);
  if (context === undefined) {
    throw new Error('useParts must be used within a PartsProvider');
  }
  return context;
};