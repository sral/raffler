import React from 'react';
import { ListGroup, Button } from 'react-bootstrap';
import { formatDateTime } from '../../utils/formatting.js';

/**
 * Notes display component with delete functionality
 */
export const Notes = React.memo(function Notes({ notes, onDeleteNote }) {
  if (!notes || notes.length === 0) {
    return null;
  }

  return (
    <ListGroup variant="flush" className="notes-container mb-3">
      {notes.map((note) => (
        <ListGroup.Item key={note.id} className="note-item d-flex justify-content-between align-items-start">
          <div className="flex-grow-1">
            <span className="fw-bold">{formatDateTime(note.created_at)}: </span>
            <span className="text-muted">{note.note}</span>
          </div>
          {onDeleteNote && (
            <Button
              variant="link"
              size="sm"
              className="text-danger p-0 ms-2"
              onClick={() => onDeleteNote(note.id)}
              aria-label="Delete note"
            >
              ×
            </Button>
          )}
        </ListGroup.Item>
      ))}
    </ListGroup>
  );
});
