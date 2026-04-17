import React from 'react';
import { Modal, Form, Button } from 'react-bootstrap';

/**
 * Modal for adding a new location
 */
export function AddLocationModal({
  show,
  onHide,
  onAddLocation,
}) {
  const [name, setName] = React.useState('');
  const [error, setError] = React.useState('');

  const handleClose = React.useCallback(() => {
    setName('');
    setError('');
    onHide();
  }, [onHide]);

  const handleSubmit = React.useCallback(async () => {
    if (!name.trim()) {
      setError('Location name is required.');
      return;
    }

    try {
      await onAddLocation(name);
      handleClose();
    } catch (err) {
      setError(err.message || 'Failed to add location');
    }
  }, [name, onAddLocation, handleClose]);

  return (
    <Modal show={show} onHide={handleClose}>
      <Modal.Header closeButton>
        <Modal.Title>Add new location</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        {error && (
          <div className="alert alert-danger" role="alert">
            {error}
          </div>
        )}
        <Form onSubmit={(e) => e.preventDefault()}>
          <Form.Group className="mb-3" controlId="formAddLocation">
            <Form.Label>Enter location name</Form.Label>
            <Form.Control
              type="text"
              placeholder="Ex: Special When Shit"
              onChange={(e) => setName(e.target.value)}
              value={name}
              aria-label="Location name"
            />
          </Form.Group>
        </Form>
      </Modal.Body>
      <Modal.Footer>
        <Button variant="secondary" onClick={handleClose}>
          Cancel
        </Button>
        <Button
          variant="primary"
          onClick={handleSubmit}
          disabled={!name.trim()}
        >
          Save
        </Button>
      </Modal.Footer>
    </Modal>
  );
}
