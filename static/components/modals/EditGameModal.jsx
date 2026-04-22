import React from 'react';
import { Modal, Form, Button } from 'react-bootstrap';

export function EditGameModal({
  show,
  onHide,
  onExited,
  game,
  onEditGame,
}) {
  const [name, setName] = React.useState(game?.name ?? '');
  const [abbreviation, setAbbreviation] = React.useState(game?.abbreviation ?? '');
  const [error, setError] = React.useState('');

  const handleClose = React.useCallback(() => {
    setError('');
    onHide();
  }, [onHide]);

  const handleSubmit = React.useCallback(async () => {
    if (!name.trim() || !abbreviation.trim()) {
      setError('Both name and abbreviation are required.');
      return;
    }

    try {
      await onEditGame(name, abbreviation);
      handleClose();
    } catch (err) {
      setError(err.message || 'Failed to update game');
    }
  }, [name, abbreviation, onEditGame, handleClose]);

  const hasChanges = game && (name !== game.name || abbreviation !== game.abbreviation);

  return (
    <Modal show={show} onHide={handleClose} onExited={onExited}>
      <Modal.Header closeButton>
        <Modal.Title>Edit game</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        {error && (
          <div className="alert alert-danger" role="alert">
            {error}
          </div>
        )}
        <Form onSubmit={(e) => e.preventDefault()}>
          <Form.Group className="mb-3" controlId="formEditGameAbbreviation">
            <Form.Label>Game abbreviation</Form.Label>
            <Form.Control
              type="text"
              placeholder="Ex: IRMA"
              value={abbreviation}
              onChange={(e) => setAbbreviation(e.target.value)}
              aria-label="Game abbreviation"
            />
          </Form.Group>

          <Form.Group className="mb-3" controlId="formEditGameName">
            <Form.Label>Game name</Form.Label>
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
          disabled={!name.trim() || !abbreviation.trim() || !hasChanges}
        >
          Save
        </Button>
      </Modal.Footer>
    </Modal>
  );
}
