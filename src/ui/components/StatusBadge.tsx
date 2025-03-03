import React from 'react';
import { PartStatus } from '../context/PartsContext';

interface StatusBadgeProps {
  status: PartStatus;
}

/**
 * StatusBadge component for displaying part status
 */
const StatusBadge: React.FC<StatusBadgeProps> = ({ status }) => {
  // Determine badge color based on status
  let bgColor = '';
  let textColor = '';

  switch (status) {
    case 'Draft':
      bgColor = 'bg-yellow-100 dark:bg-yellow-900';
      textColor = 'text-yellow-800 dark:text-yellow-200';
      break;
    case 'In Review':
      bgColor = 'bg-blue-100 dark:bg-blue-900';
      textColor = 'text-blue-800 dark:text-blue-200';
      break;
    case 'Released':
      bgColor = 'bg-green-100 dark:bg-green-900';
      textColor = 'text-green-800 dark:text-green-200';
      break;
    case 'Obsolete':
      bgColor = 'bg-gray-100 dark:bg-gray-700';
      textColor = 'text-gray-800 dark:text-gray-300';
      break;
    default:
      bgColor = 'bg-gray-100 dark:bg-gray-700';
      textColor = 'text-gray-800 dark:text-gray-300';
  }

  return (
    <span
      className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${bgColor} ${textColor}`}
    >
      {status}
    </span>
  );
};

export default StatusBadge;