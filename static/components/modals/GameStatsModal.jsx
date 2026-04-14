import React from 'react';
import { Modal, Container, Row, Col, Table } from 'react-bootstrap';
import { formatDuration } from '../../utils/formatting.js';

/**
 * Modal for displaying game reservation statistics
 */
export const GameStatsModal = React.memo(function GameStatsModal({
  show,
  onHide,
  onExited,
  stats,
}) {
  return (
    <Modal show={show} onHide={onHide} onExited={onExited}>
      {stats && (
        <>
          <Modal.Header closeButton>
            <Modal.Title>Game Statistics</Modal.Title>
          </Modal.Header>
          <Modal.Body>
            <Container>
              <Row>
                <Col>
                  <h5>Reservation Statistics</h5>
                  <Table striped bordered hover>
                    <tbody>
                      <tr>
                        <td>Total Reservations</td>
                        <td>{stats.reservation_count}</td>
                      </tr>
                      <tr>
                        <td>Total Reserved Time</td>
                        <td>{formatDuration(stats.reserved_minutes)}</td>
                      </tr>
                      <tr>
                        <td>Average Reservation Time</td>
                        <td>{Math.round(stats.average_reserved_minutes)} minutes</td>
                      </tr>
                      <tr>
                        <td>Median Reservation Time</td>
                        <td>{Math.round(stats.median_reserved_minutes)} minutes</td>
                      </tr>
                    </tbody>
                  </Table>
                </Col>
              </Row>
            </Container>
          </Modal.Body>
        </>
      )}
    </Modal>
  );
});
