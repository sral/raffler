import React from 'react';
import { Modal, Button } from 'react-bootstrap';

/**
 * Reusable confirmation dialog modal
 */
export function ConfirmationModal({
  show,
  onHide,
  onConfirm,
  title = 'Confirm Action',
  message = 'Are you sure?',
  confirmText = 'Confirm',
  cancelText = 'Cancel',
  variant = 'danger',
}) {
  const handleConfirm = () => {
    onConfirm();
    onHide();
  };

  return (
    <Modal show={show} onHide={onHide} centered>
      <Modal.Header closeButton>
        <Modal.Title>{title}</Modal.Title>
      </Modal.Header>
      <Modal.Body>{message}</Modal.Body>
      <Modal.Footer>
        <Button variant="secondary" onClick={onHide}>
          {cancelText}
        </Button>
        <Button variant={variant} onClick={handleConfirm}>
          {confirmText}
        </Button>
      </Modal.Footer>
    </Modal>
  );
}

/**
 * Custom hook for managing confirmation dialogs
 */
export function useConfirmation() {
  const [confirmState, setConfirmState] = React.useState({
    show: false,
    title: '',
    message: '',
    onConfirm: null,
    confirmText: 'Confirm',
    cancelText: 'Cancel',
    variant: 'danger',
  });

  const showConfirmation = React.useCallback((options) => {
    setConfirmState({
      show: true,
      title: options.title || 'Confirm Action',
      message: options.message || 'Are you sure?',
      onConfirm: options.onConfirm,
      confirmText: options.confirmText || 'Confirm',
      cancelText: options.cancelText || 'Cancel',
      variant: options.variant || 'danger',
    });
  }, []);

  const hideConfirmation = React.useCallback(() => {
    setConfirmState((prev) => ({ ...prev, show: false }));
  }, []);

  const confirmationDialog = React.useMemo(() => (
    <ConfirmationModal
      show={confirmState.show}
      onHide={hideConfirmation}
      onConfirm={confirmState.onConfirm}
      title={confirmState.title}
      message={confirmState.message}
      confirmText={confirmState.confirmText}
      cancelText={confirmState.cancelText}
      variant={confirmState.variant}
    />
  ), [confirmState, hideConfirmation]);

  return {
    showConfirmation,
    hideConfirmation,
    confirmationDialog,
  };
}
