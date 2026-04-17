import React from 'react';
import { Modal, Form, Button } from 'react-bootstrap';
import { Notes } from '../shared/Notes.jsx';
import { useConfirmation } from './ConfirmationModal.jsx';

/**
 * Modal for viewing and adding game details/notes
 */
export function GameDetailsModal({
  show,
  onHide,
  onExited,
  game,
  onAddNote,
  onDeleteNote,
}) {
  const [noteText, setNoteText] = React.useState('');
  const { showConfirmation, confirmationDialog } = useConfirmation();

  const handleSubmit = React.useCallback(async (event) => {
    event.preventDefault();
    if (noteText.trim()) {
      try {
        await onAddNote(noteText);
        setNoteText('');
      } catch {
        // Error already shown via toast; keep noteText for retry
      }
    }
  }, [noteText, onAddNote]);

  const handleDeleteNote = React.useCallback((noteId) => {
    showConfirmation({
      title: 'Delete Note',
      message: 'Are you sure you want to delete this note?',
      confirmText: 'Delete',
      variant: 'danger',
      onConfirm: () => onDeleteNote(noteId),
    });
  }, [showConfirmation, onDeleteNote]);

  const handleClose = React.useCallback(() => {
    setNoteText('');
    onHide();
  }, [onHide]);

  return (
    <Modal show={show} onHide={handleClose} onExited={onExited}>
      {game && (
        <>
          <Modal.Header closeButton>
            <Modal.Title>{game.name} ({game.abbreviation})</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            {confirmationDialog}
            <Notes notes={game.notes} onDeleteNote={handleDeleteNote} />
            <Form onSubmit={handleSubmit}>
              <Form.Group className="mb-3" controlId="formAddNote">
                <div className="d-flex gap-2">
                  <Form.Control
                    type="text"
                    placeholder="Ex: Autoplunger is infested with cows"
                    onChange={(e) => setNoteText(e.target.value)}
                    value={noteText}
                    aria-label="Note text"
                  />
                  <Button variant="primary" type="submit" disabled={!noteText.trim()}>
                    Save
                  </Button>
                </div>
              </Form.Group>
            </Form>
          </Modal.Body>
        </>
      )}
    </Modal>
  );
}
