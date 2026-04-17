import React from 'react';
import { Spinner } from 'react-bootstrap';

/**
 * Loading spinner component
 */
export function LoadingSpinner({ size = 'sm', className = '' }) {
  return (
    <Spinner
      animation="border"
      size={size}
      role="status"
      className={className}
    >
      <span className="visually-hidden">Loading...</span>
    </Spinner>
  );
}

/**
 * Full page loading spinner
 */
export function PageLoader() {
  return (
    <div className="d-flex justify-content-center align-items-center" style={{ minHeight: '200px' }}>
      <Spinner animation="border" role="status">
        <span className="visually-hidden">Loading...</span>
      </Spinner>
    </div>
  );
}
