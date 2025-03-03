import React, { createContext, useContext, useState, ReactNode } from 'react';

// Define user roles
export type UserRole = 'admin' | 'engineer' | 'viewer';

// Define user interface
export interface User {
  id: string;
  name: string;
  email: string;
  role: UserRole;
}

// Define auth context type
interface AuthContextType {
  user: User | null;
  isAuthenticated: boolean;
  login: (email: string, password: string) => Promise<void>;
  logout: () => void;
  hasPermission: (permission: string) => boolean;
}

// Create the context
const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

/**
 * Provider component for authentication state
 */
export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);

  // Mock login function - in a real app, this would call the backend
  const login = async (email: string, password: string) => {
    // This is a placeholder for actual authentication logic
    // In a real app, this would make an API call to authenticate
    const mockUser: User = {
      id: '1',
      name: 'Test User',
      email: email,
      role: 'engineer',
    };
    
    setUser(mockUser);
  };

  const logout = () => {
    setUser(null);
  };

  // Check if user has a specific permission
  const hasPermission = (permission: string): boolean => {
    if (!user) return false;
    
    // This is a simple permission check based on role
    // In a real app, this would be more sophisticated
    switch (permission) {
      case 'create_part':
        return ['admin', 'engineer'].includes(user.role);
      case 'approve_part':
        return ['admin'].includes(user.role);
      case 'view_part':
        return ['admin', 'engineer', 'viewer'].includes(user.role);
      default:
        return false;
    }
  };

  const value = {
    user,
    isAuthenticated: !!user,
    login,
    logout,
    hasPermission,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

/**
 * Hook for accessing auth context
 */
export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};