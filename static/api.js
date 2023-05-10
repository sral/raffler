const API_URL = 'http://localhost:8000';

const request = async (url, options) => {
  const response = await fetch(url, options);
  if (!response.ok) {
    throw new Error(`Error: ${response.status}`);
  }
  
  return response.json()
  .catch((error) => {
    // TODO: Ugly handling of enpoints which do not return json.
    return null;
  });
};

export const API = {
  games: {
    get: async (locationId, gameId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}`;
      const options = {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      };
      return request(url, options);
    },

    getAll: async (locationId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games`;
      const options = {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      };
      return request(url, options);
    },
    
    reserveRandom: async (locationId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/reservations`;
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      };
      return request(url, options);
    },
    
    reserve: async (locationId, gameId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}/reservations`;
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      };
      return request(url, options);
    },
    
    release: async (locationId, gameId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}/reservations`;
      const options = {
        method: 'DELETE',
      };
      return request(url, options);
    },
    
    remove: async (locationId, gameId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}`;
      const options = {
        method: 'DELETE',
      };
      return request(url, options);
    },
    
    enable: async (locationId, gameId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}/enable`;
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      };
      return request(url, options);
    },
    
    disable: async (locationId, gameId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}/disable`;
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      };
      return request(url, options);
    },
    
    add: async (locationId, name, abbreviation) => {
      const url = `${API_URL}/v1/locations/${locationId}/games`;
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name,
          abbreviation,
        }),
      };
      return request(url, options);
    },
    
    update: async (locationId, gameId, name, abbreviation) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}`;
      const options = {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name,
          abbreviation,
        }),
      };
      return request(url, options);
    },
  },
  
  locations: {
    getAll: async () => {
      const url = `${API_URL}/v1/locations`;
      const options = {
        method: 'GET',
        headers: {
          'Content-Type': 'application/json',
        },
      };
      return request(url, options);
    },
    
    add: async (name) => {
      const url = `${API_URL}/v1/locations`;
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          name,
        }),
      };
      return request(url, options);
    }
  },

  notes: {
    add: async (locationId, gameId, note) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}/notes`;
      const options = {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          note,
        }),
      };
      return request(url, options);
    },

    remove: async (locationId, gameId, noteId) => {
      const url = `${API_URL}/v1/locations/${locationId}/games/${gameId}/notes/${noteId}`;
      const options = {
        method: 'DELETE',
      };
      return request(url, options);
    },
  }
};
