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
  const outlierCount = stats
    ? stats.total_reservation_count - stats.analysed_reservation_count
    : 0;

  const p25 = Math.round(stats?.p25_reserved_minutes ?? 0);
  const p50 = Math.round(stats?.median_reserved_minutes ?? 0);
  const p75 = Math.round(stats?.p75_reserved_minutes ?? 0);

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
                        <td>Reservations</td>
                        <td>{stats.total_reservation_count}</td>
                      </tr>
                      <tr>
                        <td>Total play time</td>
                        <td>{formatDuration(stats.total_reserved_minutes)}</td>
                      </tr>
                      <tr>
                        <td>Typical session</td>
                        <td>
                          {stats.analysed_reservation_count > 0
                            ? `${p25}–${p75} min (median ${p50} min)`
                            : '—'}
                        </td>
                      </tr>
                    </tbody>
                  </Table>
                  {outlierCount > 0 && (
                    <p className="text-muted small mb-0">
                      Typical session excludes {outlierCount} outlier{outlierCount === 1 ? '' : 's'}.
                    </p>
                  )}
                </Col>
              </Row>
            </Container>
          </Modal.Body>
        </>
      )}
    </Modal>
  );
});
