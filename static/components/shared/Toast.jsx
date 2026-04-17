import React from 'react';
import { Toast, ToastContainer } from 'react-bootstrap';
import { TOAST_SUCCESS_DURATION_MS, TOAST_ERROR_DURATION_MS, SHOW_SUCCESS_TOASTS } from '../../constants.js';

/**
 * Toast notification component for displaying temporary messages
 */
export function ToastNotification({ show, onClose, message, variant = 'danger', delay = 5000 }) {
  return (
    <ToastContainer position="top-end" className="p-3" style={{ zIndex: 9999 }}>
      <Toast show={show} onClose={onClose} delay={delay} autohide bg={variant}>
        <Toast.Header>
          <strong className="me-auto">
            {variant === 'danger' ? 'Error' : variant === 'success' ? 'Success' : 'Info'}
          </strong>
        </Toast.Header>
        <Toast.Body className={variant === 'danger' ? 'text-white' : ''}>
          {message}
        </Toast.Body>
      </Toast>
    </ToastContainer>
  );
}

/**
 * Custom hook for managing toast notifications
 */
let nextToastId = 0;

export function useToast() {
  const [toasts, setToasts] = React.useState([]);

  const showToast = React.useCallback((message, variant = 'danger') => {
    const id = nextToastId++;
    setToasts((prev) => [...prev, { id, message, variant }]);
  }, []);

  const hideToast = React.useCallback((id) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  }, []);

  const showError = React.useCallback((message, error) => {
    const errorMessage = error?.message ? `${message}: ${error.message}` : message;
    showToast(errorMessage, 'danger');
  }, [showToast]);

  const showSuccess = React.useCallback((message) => {
    if (SHOW_SUCCESS_TOASTS) {
      showToast(message, 'success');
    }
  }, [showToast]);

  const toastList = React.useMemo(() => {
    return (
      <ToastContainer position="top-end" className="p-3" style={{ zIndex: 9999 }}>
        {toasts.map((toast) => {
          // Success toasts auto-close after 1 second, errors after 5 seconds
          const delay = toast.variant === 'success' ? TOAST_SUCCESS_DURATION_MS : TOAST_ERROR_DURATION_MS;

          return (
            <Toast
              key={toast.id}
              show={true}
              onClose={() => hideToast(toast.id)}
              delay={delay}
              autohide
              bg={toast.variant}
            >
              <Toast.Header>
                <strong className="me-auto">
                  {toast.variant === 'danger' ? 'Error' : toast.variant === 'success' ? 'Success' : 'Info'}
                </strong>
              </Toast.Header>
              <Toast.Body className={toast.variant === 'danger' ? 'text-white' : ''}>
                {toast.message}
              </Toast.Body>
            </Toast>
          );
        })}
      </ToastContainer>
    );
  }, [toasts, hideToast]);

  return {
    showToast,
    showError,
    showSuccess,
    toastList,
  };
}
