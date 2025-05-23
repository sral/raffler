import {API} from './api.js';

// TODO: Clean this crap up once imports are fixed.
const Button = ReactBootstrap.Button;
const ButtonGroup = ReactBootstrap.ButtonGroup;
const Col = ReactBootstrap.Col;
const Container = ReactBootstrap.Container;
const Dropdown = ReactBootstrap.Dropdown;
const DropdownButton = ReactBootstrap.DropdownButton;
const Form = ReactBootstrap.Form;
const ListGroup = ReactBootstrap.ListGroup;
const Modal = ReactBootstrap.Modal;
const Nav = ReactBootstrap.Nav;
const Navbar = ReactBootstrap.Navbar;
const NavDropdown = ReactBootstrap.NavDropdown;
const OverlayTrigger = ReactBootstrap.OverlayTrigger;
const Row = ReactBootstrap.Row;
const Tooltip = ReactBootstrap.Tooltip;
const Table = ReactBootstrap.Table;

const Event = {
  Disable: 'disable',
  Update: 'update',
  Remove: 'remove',
  Details: 'details',
  Stats: 'stats',
};

function formatDuration(minutes) {
  const days = Math.floor(minutes / (24 * 60));
  const hours = Math.floor((minutes % (24 * 60)) / 60);
  const mins = minutes % 60;

  const parts = [];
  if (days > 0) parts.push(`${days} day${days === 1 ? '' : 's'}`);
  if (hours > 0) parts.push(`${hours} hour${hours === 1 ? '' : 's'}`);
  if (mins > 0) parts.push(`${mins} minute${mins === 1 ? '' : 's'}`);

  if (parts.length === 0) return '0 minutes';
  if (parts.length === 1) return parts[0];
  if (parts.length === 2) return `${parts[0]} and ${parts[1]}`;
  return `${parts[0]}, ${parts[1]} and ${parts[2]}`;
}

function Notes({ notes }) {
  const formatDate = (timestamp) => {
    const [date, time] = timestamp.replace('T', ' ').split(' ');
    const [hour, minute] = time.split(':');
    return `${date} ${hour}:${minute}`;
  };

  return (
    <ListGroup variant="flush" className="notes-container">
      {notes.map((note) => (
        <ListGroup.Item key={note.id} className="note-item">
          <span>{formatDate(note.created_at)}: </span>
          <span className="text-muted">{note.note}</span>
        </ListGroup.Item>
      ))}
    </ListGroup>
  );
}

function GameDetailsModal({
  gameDetails,
  modalGameDetailsShow,
  setGameDetails,
  setModalGameDetailsShow,
  selectedLocation,
}) {
  const [value, setValue] = React.useState('');

  const onAddNote = async () => {
    if (value.trim() !== '') {
      const addedNote = await API.notes.add(selectedLocation.id, gameDetails.id, value);

      setGameDetails((prevDetails) => {
        const updatedDetails = { ...prevDetails };
        if (!updatedDetails.notes) {
          updatedDetails.notes = []; // Initialize notes array if it's empty
        }
        updatedDetails.notes.push(addedNote);
        return updatedDetails;
      });
    }

    setValue('');
  };

  const onInput = (event) => {
    const inputValue = event.target.value;
    setValue(inputValue);
  };

  const onSubmit = (event) => {
    event.preventDefault();
    onAddNote();
  };

  const onClose = () => {
    setModalGameDetailsShow(false);
    setGameDetails(null);
  };

  return (
    <Modal show={modalGameDetailsShow} onHide={onClose}>
      <Modal.Header closeButton>
        <Modal.Title>{gameDetails?.name || ''} ({gameDetails?.abbreviation || ''})</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        {gameDetails?.notes && gameDetails.notes.length > 0 && <Notes notes={gameDetails.notes} />}
        <Form onSubmit={onSubmit}>
          <Form.Group className="mb-3" controlId="formAddNote">
            <div className="d-flex">
              <Form.Control
                type="text"
                placeholder="Ex: Autoplunger is infested with cows"
                onChange={onInput}
                value={value}
              />
              <Button variant="primary" type="submit">Save</Button>
            </div>
          </Form.Group>
        </Form>
      </Modal.Body>
    </Modal>
  );
}

function GameStatsModal({
  modalGameStatsShow,
  setModalGameStatsShow,
  gameStats,
}) {
  if (!gameStats) {
    return null;
  }

  return (
    <Modal show={modalGameStatsShow} onHide={() => setModalGameStatsShow(false)}>
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
                    <td>{gameStats.reservation_count}</td>
                  </tr>
                  <tr>
                    <td>Total Reserved Time</td>
                    <td>{formatDuration(gameStats.reserved_minutes)}</td>
                  </tr>
                  <tr>
                    <td>Average Reservation Time</td>
                    <td>{Math.round(gameStats.average_reserved_minutes)} minutes</td>
                  </tr>
                  <tr>
                    <td>Median Reservation Time</td>
                    <td>{Math.round(gameStats.median_reserved_minutes)} minutes</td>
                  </tr>
                </tbody>
              </Table>
            </Col>
          </Row>
        </Container>
      </Modal.Body>
    </Modal>
  );
}

function AddLocationModal({
  modalAddLocationShow,
  setModalAddLocationShow,
  setLocations,
  setSelectedLocation,
  setReservedGame,
}) {
  const [value, setValue] = React.useState('');

  const onInput = (event) => setValue(event.target.value);
  const onSubmit = (event) => event.preventDefault();
  const onClose = () => {
    setValue('');
    setModalAddLocationShow(false);
  };

  async function onAddLocation(event) {
    if (value) {
      try {
        const newLocation = await API.locations.add(value);
        setSelectedLocation(newLocation);
        setLocations((locations) => [...locations, newLocation]);
        setValue('');
        setReservedGame(null);
        setModalAddLocationShow(false);
      } catch (error) {
        console.error('Error adding location', error);
      }
    }
  }

  return (
    <Modal show={modalAddLocationShow} onHide={onClose}>
      <Modal.Header closeButton>
        <Modal.Title>Add new location</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <Form onSubmit={onSubmit}>
          <Form.Group className="mb-3" controlId="formAddLocation">
            <Form.Label>Enter location name</Form.Label>
            <Form.Control
              type="text"
              placeholder="Ex: Special When Shit"
              onChange={onInput}
              value={value}
            />
          </Form.Group>
        </Form>
      </Modal.Body>
      <Modal.Footer>
        <Button variant="secondary" onClick={onClose}>
          Cancel
        </Button>
        <Button variant="primary" onClick={onAddLocation}>
          Save
        </Button>
      </Modal.Footer>
    </Modal>
  );
}

function AddGameModal({ modalAddGameShow, selectedLocation, setModalAddGameShow, setGameStates }) {
  const [name, setName] = React.useState('');
  const [abbreviation, setAbbreviation] = React.useState('');

  const handleInputName = (event) => setName(event.target.value);
  const handleInputAbbreviation = (event) => setAbbreviation(event.target.value);
  const handleClose = () => {
    setName('');
    setAbbreviation('');
    setModalAddGameShow(false);
  };

  const handleAddGame = async () => {
    if (!name.trim() || !abbreviation.trim()) {
      console.error('Both name and abbreviation are required.');
      return;
    }

    try {
      await API.games.add(selectedLocation.id, name, abbreviation);
      setGameStates(await API.games.getAll(selectedLocation.id));
      setName('');
      setAbbreviation('');
      setModalAddGameShow(false);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <Modal show={modalAddGameShow} onHide={handleClose}>
      <Modal.Header closeButton>
        <Modal.Title>Add game</Modal.Title>
      </Modal.Header>
      <Modal.Body>
        <Form onSubmit={(event) => event.preventDefault()}>
          <Form.Group className="mb-3" controlId="formAddGameAbbreviation">
            <Form.Label>Enter game abbreviation</Form.Label>
            <Form.Control
              type="text"
              placeholder="Ex: IRMA"
              value={abbreviation}
              onChange={handleInputAbbreviation}
            />
          </Form.Group>

          <Form.Group className="mb-3" controlId="formAddGameName">
            <Form.Label>Enter game name</Form.Label>
            <Form.Control
              type="text"
              placeholder="Ex: Iron Maiden (Stern)"
              value={name}
              onChange={handleInputName}
            />
          </Form.Group>
        </Form>
      </Modal.Body>
      <Modal.Footer>
        <Button variant="secondary" onClick={handleClose}>
          Cancel
        </Button>
        <Button variant="primary" onClick={handleAddGame}>
          Save
        </Button>
      </Modal.Footer>
    </Modal>
  );
}

function LocationPicker({ locations, selectedLocation, onSelectLocationClick, onAddLocationClick, onAddGameClick }) {
  const renderAddGameButton = () => {
    if (!selectedLocation) {
      return null;
    }

    return (
      <Nav.Link
        href="#"
        variant="outline-secondary"
        onClick={onAddGameClick}
      >
        Add game
      </Nav.Link>
    );
  };

  const renderLocationsDropdown = () => (
    <NavDropdown title="Locations" id="basic-nav-dropdown">
      {locations.map(location => (
        <NavDropdown.Item
          key={location.id}
          href="#"
          onClick={() => onSelectLocationClick(location)}
        >
          {location.name}
        </NavDropdown.Item>
      ))}
      <NavDropdown.Divider />
      <NavDropdown.Item href="#" onClick={onAddLocationClick}>
        Add new location
      </NavDropdown.Item>
    </NavDropdown>
  );

  return (
    <Navbar bg="light" expand="lg">
      <Container>
        <Navbar.Brand>{selectedLocation?.name}</Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
        <Navbar.Collapse id="basic-navbar-nav">
          <Nav className="ms-auto">
            {renderAddGameButton()}
            {renderLocationsDropdown()}
          </Nav>
        </Navbar.Collapse>
      </Container>
    </Navbar>
  );
}

function RandomizeButton({ onRaffleClick }) {
  return (
    <Button variant='primary' onClick={onRaffleClick} size='lg' className='fixed-width-button mx-1 my-2'>
      Randomize!
    </Button>
  );
}

function GameButton({ game, onButtonClick, onToggleGameDisabledClick, onRemoveGameClick, onGameDetailsClick, onGameStatsClick }) {
  const { name, abbreviation, disabled_at, reserved_at, reserved_minutes } = game;

  const isDisabled = Boolean(disabled_at);
  const isReserved = Boolean(reserved_at);

  const getButtonVariant = () => {
    if (isDisabled) return 'secondary';
     if (!isReserved) return 'success';
    // If reserved for more than 30 minutes, show yellow (warning)
    if (reserved_minutes > 30) return 'warning';
    // Otherwise show red (danger)
    return 'danger';
  };

  const buttonVariant = getButtonVariant();  const buttonText = isReserved ? `${abbreviation} (${reserved_minutes}m)` : abbreviation;

  const handleButtonClick = () => {
    if (!isDisabled) {
      onButtonClick();
    }
  };

  return (
    <OverlayTrigger placement='top' overlay={<Tooltip><strong>{name}</strong></Tooltip>}>
      <ButtonGroup key={`buttongroup-${game.id}`} className="fixed-width-button mx-1 my-2">
        <Button title={name} variant={buttonVariant} disabled={isDisabled} onClick={handleButtonClick}>
          {buttonText}
        </Button>
        <DropdownButton variant={buttonVariant} as={ButtonGroup} id='bg-nested-dropdown' drop='end'>
          <Dropdown.Item eventKey={Event.Disable} onClick={onToggleGameDisabledClick}>
            {isDisabled ? 'Enable' : 'Disable'}
          </Dropdown.Item>
          <Dropdown.Item eventKey={Event.Remove} onClick={onRemoveGameClick}>Remove</Dropdown.Item>
          <Dropdown.Item eventKey={Event.Details} onClick={onGameDetailsClick}>Details</Dropdown.Item>
          <Dropdown.Item eventKey={Event.Stats} onClick={onGameStatsClick}>Stats</Dropdown.Item>
        </DropdownButton>
      </ButtonGroup>
    </OverlayTrigger>
  );
}

function GameList({ gameStates, selectedLocation, onGameClick, onToggleGameDisabledClick, onRemoveGameClick, onGameDetailsClick, onGameStatsClick }) {
  return (
    <Container fluid='md'>
      <Row>
        <Col>
          {gameStates.map((game, index) => (
            <GameButton
              key={game.id}
              game={game}
              onButtonClick={() => onGameClick(index)}
              onToggleGameDisabledClick={() => onToggleGameDisabledClick(selectedLocation, game)}
              onRemoveGameClick={() => onRemoveGameClick(selectedLocation, game)}
              onGameDetailsClick={() => onGameDetailsClick(selectedLocation, game)}
              onGameStatsClick={() => onGameStatsClick(selectedLocation, game)}
            />
          ))}
        </Col>
      </Row>
    </Container>
  );
}

function Raffler() {
  const [locations, setLocations] = React.useState([]);
  const [selectedLocation, setSelectedLocation] = React.useState(null);
  const [reservedGame, setReservedGame] = React.useState(null);
  const [gameStates, setGameStates] = React.useState([]);
  const [gameDetails, setGameDetails] = React.useState(null);
  const [gameStats, setGameStats] = React.useState(null);

  // Modals
  const [modalAddLocationShow, setModalAddLocationShow] = React.useState(false);
  const [modalAddGameShow, setModalAddGameShow] = React.useState(false);
  const [modalGameDetailsShow, setModalGameDetailsShow] = React.useState(false);
  const [modalGameStatsShow, setModalGameStatsShow] = React.useState(false);

  React.useEffect(() => {
    const getLocations = async () => {
      setLocations(await API.locations.getAll());
    }

    getLocations();
  }, []);

  React.useEffect(() => {
    const getGameStates = async () => {
      if (selectedLocation) {
        setGameStates(await API.games.getAll(selectedLocation.id));
      }
    }

    getGameStates();
  }, [selectedLocation]);


  async function onRandomizeClick() {
    setReservedGame(await API.games.reserveRandom(selectedLocation.id));
    // Wasteful! This round-trip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.games.getAll(selectedLocation.id));
  }

  async function onGameClick(i) {
    const game = gameStates[i];

    if (game.reserved_at) {
      await API.games.release(selectedLocation.id, game.id);
    } else {
      await API.games.reserve(selectedLocation.id, game.id);
      setReservedGame(game);
    }
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.games.getAll(selectedLocation.id));
  }

  function onSelectLocationClick(location) {
    if (selectedLocation && location.id === selectedLocation.id) {
      // Don't update if location doesn't change.
      return;
    }

    setReservedGame(null);
    setSelectedLocation(location);
  }

  function onAddLocationClick() {
    setModalAddLocationShow(true);
  }

  function onAddGameClick() {
    setModalAddGameShow(true);
  }

  async function onGameDetailsClick(location, game) {
    setGameDetails(await API.games.get(location.id, game.id));
    setModalGameDetailsShow(true);
  }

  async function onRemoveGameClick(location, game) {
    if (!window.confirm("Are you sure?")) {
      return;
    }

    await API.games.remove(location.id, game.id);
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.games.getAll(selectedLocation.id));
  }

  async function onToggleGameDisabledClick(location, game) {
    const isDisabled = Boolean(game.disabled_at);
    const isReserved = Boolean(game.reserved_at);

    if (isDisabled) {
      await API.games.enable(location.id, game.id);
    } else {
      if (isReserved) {
        await API.games.release(location.id, game.id);
      }
      await API.games.disable(location.id, game.id);
    }
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.games.getAll(selectedLocation.id));
  }

  async function onGameStatsClick(location, game) {
    try {
      const stats = await API.games.getStats(location.id, game.id);
      setGameStats(stats);
      setModalGameStatsShow(true);
    } catch (error) {
      console.error('Error fetching game stats:', error);
      // Optionally show an error message to the user
    }
  }

  return (
    <div className="container-fluid">
      <header>
        <LocationPicker
        locations={locations}
        selectedLocation={selectedLocation}
        onSelectLocationClick={onSelectLocationClick}
        onAddLocationClick={onAddLocationClick}
        onAddGameClick={onAddGameClick}
        />
      </header>

      <div  className={selectedLocation ? 'visible': 'invisible'}>
        <div className="text-center my-2">
          <RandomizeButton
            onRaffleClick={onRandomizeClick}
          />
        </div>
        <div className="fixed-height-selected-game my-2">
            <h3>
              {reservedGame ? reservedGame.name : ''}
            </h3>
        </div>
        <div>
          <GameList
            gameStates={gameStates}
            selectedLocation={selectedLocation}
            onGameClick={onGameClick}
            onToggleGameDisabledClick={onToggleGameDisabledClick}
            onRemoveGameClick={onRemoveGameClick}
            onGameDetailsClick={onGameDetailsClick}
            onGameStatsClick={onGameStatsClick}
          />
        </div>
      </div>
      <AddLocationModal
        modalAddLocationShow={modalAddLocationShow}
        setModalAddLocationShow={setModalAddLocationShow}
        setLocations={setLocations}
        setSelectedLocation={setSelectedLocation}
        setReservedGame={setReservedGame}
      />
      <AddGameModal
        modalAddGameShow={modalAddGameShow}
        selectedLocation={selectedLocation}
        setModalAddGameShow={setModalAddGameShow}
        setGameStates={setGameStates}
      />
      <GameDetailsModal
        gameDetails={gameDetails}
        modalGameDetailsShow={modalGameDetailsShow}
        setGameDetails={setGameDetails}
        setModalGameDetailsShow={setModalGameDetailsShow}
        selectedLocation={selectedLocation}
      />
      <GameStatsModal
        modalGameStatsShow={modalGameStatsShow}
        setModalGameStatsShow={setModalGameStatsShow}
        gameStats={gameStats}
      />
    </div>
  );
}

const container = document.getElementById('root');
const root = ReactDOM.createRoot(container);
root.render(<Raffler />);
