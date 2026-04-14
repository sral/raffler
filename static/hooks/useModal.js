import React from 'react';

/**
 * Custom hook for managing modal state
 * @returns {Object} Modal state and control functions
 */
export function useModal() {
  const [show, setShow] = React.useState(false);

  const open = React.useCallback(() => setShow(true), []);
  const close = React.useCallback(() => setShow(false), []);

  return {
    show,
    open,
    close,
  };
}
