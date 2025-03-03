import React, { createContext, useContext, useState, useCallback, ReactNode } from 'react';

// Define notification types
export type NotificationType = 'info' | 'success' | 'warning' | 'error';

// Define notification interface
export interface Notification {
  id: string;
  type: NotificationType;
  message: string;
  title?: string;
  autoClose?: boolean;
  duration?: number;
}

// Define notification context type
interface NotificationContextType {
  notifications: Notification[];
  addNotification: (notification: Omit<Notification, 'id'>) => void;
  removeNotification: (id: string) => void;
  clearNotifications: () => void;
}

// Create the context
const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

interface NotificationProviderProps {
  children: ReactNode;
}

/**
 * Provider component for notification state
 */
export const NotificationProvider: React.FC<NotificationProviderProps> = ({ children }) => {
  const [notifications, setNotifications] = useState<Notification[]>([]);

  // Add a new notification
  const addNotification = useCallback((notification: Omit<Notification, 'id'>) => {
    const id = Math.random().toString(36).substring(2, 9);
    const newNotification: Notification = {
      id,
      autoClose: true,
      duration: 5000,
      ...notification,
    };

    setNotifications((prevNotifications) => [...prevNotifications, newNotification]);

    // Auto-close notification if specified
    if (newNotification.autoClose) {
      setTimeout(() => {
        removeNotification(id);
      }, newNotification.duration);
    }
  }, []);

  // Remove a notification by ID
  const removeNotification = useCallback((id: string) => {
    setNotifications((prevNotifications) =>
      prevNotifications.filter((notification) => notification.id !== id)
    );
  }, []);

  // Clear all notifications
  const clearNotifications = useCallback(() => {
    setNotifications([]);
  }, []);

  const value = {
    notifications,
    addNotification,
    removeNotification,
    clearNotifications,
  };

  return <NotificationContext.Provider value={value}>{children}</NotificationContext.Provider>;
};

/**
 * Hook for accessing notification context
 */
export const useNotifications = (): NotificationContextType => {
  const context = useContext(NotificationContext);
  if (context === undefined) {
    throw new Error('useNotifications must be used within a NotificationProvider');
  }
  return context;
};