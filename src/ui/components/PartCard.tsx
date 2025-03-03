import React from 'react';
import { Link } from 'react-router-dom';
import { Part } from '../context/PartsContext';
import StatusBadge from './StatusBadge';

interface PartCardProps {
  part: Part;
}

/**
 * PartCard component for displaying part summary information
 */
const PartCard: React.FC<PartCardProps> = ({ part }) => {
  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden hover:shadow-md transition-shadow">
      <div className="p-4">
        <div className="flex justify-between items-start">
          <div>
            <Link
              to={`/parts/${part.id}`}
              className="text-lg font-medium text-gray-900 dark:text-white hover:text-blue-600 dark:hover:text-blue-400"
            >
              {part.name}
            </Link>
            <p className="text-sm text-gray-500 dark:text-gray-400">{part.displayId}</p>
          </div>
          <StatusBadge status={part.status} />
        </div>

        <p className="mt-2 text-sm text-gray-600 dark:text-gray-300 line-clamp-2">
          {part.description || 'No description provided'}
        </p>

        <div className="mt-4 flex justify-between items-center">
          <div className="text-xs text-gray-500 dark:text-gray-400">
            <span className="block">Category: {part.category}</span>
            <span className="block">Subcategory: {part.subcategory}</span>
          </div>
          <div className="text-xs text-gray-500 dark:text-gray-400 text-right">
            <span className="block">Created: {new Date(part.createdAt).toLocaleDateString()}</span>
            <span className="block">Updated: {new Date(part.updatedAt).toLocaleDateString()}</span>
          </div>
        </div>
      </div>

      <div className="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-750 px-4 py-3 flex justify-between">
        <Link
          to={`/parts/${part.id}`}
          className="text-sm text-blue-600 dark:text-blue-400 hover:underline"
        >
          View Details
        </Link>
        <Link
          to={`/parts/${part.id}/edit`}
          className="text-sm text-gray-600 dark:text-gray-400 hover:underline"
        >
          Edit
        </Link>
      </div>
    </div>
  );
};

export default PartCard;