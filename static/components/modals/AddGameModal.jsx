import React from 'react';
import { Modal, Form, Button } from 'react-bootstrap';

/**
 * Modal for adding a new game to a location
 */
export function AddGameModal({
  show,
  onHide,
  onAddGame,
}) {
  const [name, setName] = React.useState('');
  const [abbreviation, setAbbreviation] = React.useState('');
  const [error, setError] = React.useState('');

  const handleClose = React.useCallback(() => {
    setName('');
    setAbbreviation('');
    setError('');
    onHide();
  }, [onHide]);

  const handleSubmit = React.useCallback(async () => {
    if (!name.trim() || !abbreviation.trim()) {
      setError('Both name and abbreviation are required.');
      return;
    }

    try {
      await onAddGame(name, abbreviation);
      handleClose();
    } catch (err) {
      setError(err.message || 'Failed to add game');
    }
  }, [name, abbreviation, onAddGame, handleClose]);

  return (
    <Modal show={show} onHide={handleClose}>
      <Modal.Header closeButton>
        <Modal.Title>Add game</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        {error && (
          <div className="alert alert-danger" role="alert">
            {error}
          </div>
        )}
        <Form onSubmit={(e) => e.preventDefault()}>
          <Form.Group className="mb-3" controlId="formAddGameAbbreviation">
            <Form.Label>Enter game abbreviation</Form.Label>
            <Form.Control
              type="text"
              placeholder="Ex: IRMA"
              value={abbreviation}
              onChange={(e) => setAbbreviation(e.target.value)}
              aria-label="Game abbreviation"
            />
          </Form.Group>

          <Form.Group className="mb-3" controlId="formAddGameName">
            <Form.Label>Enter game name</Form.Label>
            <Form.Control
              type="text"
              placeholder="Ex: Iron Maiden (Stern)"
              value={name}
              onChange={(e) => setName(e.target.value)}
              aria-label="Game name"
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
          disabled={!name.trim() || !abbreviation.trim()}
        >
          Save
        </Button>
      </Modal.Footer>
    </Modal>
  );
}
