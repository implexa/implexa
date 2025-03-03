import React from 'react';
import { useNotifications } from '../context/NotificationContext';

/**
 * Notifications component for displaying application notifications
 */
const Notifications: React.FC = () => {
  const { notifications, removeNotification } = useNotifications();

  if (notifications.length === 0) {
    return null;
  }

  return (
    <div className="fixed bottom-4 right-4 z-50 space-y-2 max-w-md">
      {notifications.map((notification) => {
        // Determine background color based on notification type
        let bgColor = 'bg-blue-500';
        let textColor = 'text-white';
        
        switch (notification.type) {
          case 'success':
            bgColor = 'bg-green-500';
            break;
          case 'warning':
            bgColor = 'bg-yellow-500';
            textColor = 'text-gray-900';
            break;
          case 'error':
            bgColor = 'bg-red-500';
            break;
          default:
            bgColor = 'bg-blue-500';
        }

        return (
          <div
            key={notification.id}
            className={`${bgColor} ${textColor} rounded-md shadow-lg overflow-hidden`}
            role="alert"
          >
            <div className="p-4 flex items-start">
              <div className="flex-1">
                {notification.title && (
                  <h3 className="font-bold">{notification.title}</h3>
                )}
                <p className="text-sm">{notification.message}</p>
              </div>
              <button
                onClick={() => removeNotification(notification.id)}
                className="ml-4 text-white opacity-70 hover:opacity-100 focus:outline-none"
                aria-label="Close notification"
              >
                ✕
              </button>
            </div>
            {/* Progress bar for auto-close notifications */}
            {notification.autoClose && (
              <div
                className="h-1 bg-white bg-opacity-30"
                style={{
                  animation: `shrink ${notification.duration}ms linear forwards`,
                }}
              />
            )}
          </div>
        );
      })}
    </div>
  );
};

export default Notifications;