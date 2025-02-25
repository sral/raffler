const API_URL = 'http://localhost:8000';
const API_VERSION = 'v1';
const buildUrl = (path) => `${API_URL}/${API_VERSION}${path}`;

const request = async (url, options) => {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), 5000);

  try {
    const response = await fetch(url, {
      ...options,
      signal: controller.signal,
    });
    
    clearTimeout(timeoutId);
    
    if (!response.ok) {
      const error = new Error(
        `API Error: ${response.status} ${response.statusText} at ${url}`
      );
      error.status = response.status;
      throw error;
    }
    
    const contentType = response.headers.get('content-type');
    if (contentType && contentType.includes('application/json')) {
      return response.json();
    }
    return null;
  } catch (error) {
    if (error.name === 'AbortError') {
      throw new Error(`Request timeout for ${url}`);
    }
    throw error;
  } finally {
    clearTimeout(timeoutId);
  }
};

const createJsonOptions = (method, body = null) => ({
  method,
  headers: {
    'Content-Type': 'application/json',
  },
  ...(body && { body: JSON.stringify(body) }),
});

export const API = {
  games: {
    get: async (locationId, gameId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}`);
      return request(url, createJsonOptions('GET'));
    },

    getAll: async (locationId) => {
      const url = buildUrl(`/locations/${locationId}/games`);
      return request(url, createJsonOptions('GET'));
    },
    
    reserveRandom: async (locationId) => {
      const url = buildUrl(`/locations/${locationId}/games/reservations`);
      return request(url, createJsonOptions('POST'));
    },
    
    reserve: async (locationId, gameId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}/reservations`);
      return request(url, createJsonOptions('POST'));
    },
    
    release: async (locationId, gameId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}/reservations`);
      return request(url, createJsonOptions('DELETE'));
    },
    
    remove: async (locationId, gameId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}`);
      return request(url, createJsonOptions('DELETE'));
    },
    
    enable: async (locationId, gameId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}/enable`);
      return request(url, createJsonOptions('POST'));
    },
    
    disable: async (locationId, gameId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}/disable`);
      return request(url, createJsonOptions('POST'));
    },
    
    add: async (locationId, name, abbreviation) => {
      const url = buildUrl(`/locations/${locationId}/games`);
      return request(url, createJsonOptions('POST', { name, abbreviation }));
    },
    
    update: async (locationId, gameId, name, abbreviation) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}`);
      return request(url, createJsonOptions('PUT', { name, abbreviation }));
    },
  },
  
  locations: {
    getAll: async () => {
      const url = buildUrl('/locations');
      return request(url, createJsonOptions('GET'));
    },
    
    add: async (name) => {
      const url = buildUrl('/locations');
      return request(url, createJsonOptions('POST', { name }));
    }
  },

  notes: {
    add: async (locationId, gameId, note) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}/notes`);
      return request(url, createJsonOptions('POST', { note }));
    },

    remove: async (locationId, gameId, noteId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}/notes/${noteId}`);
      return request(url, createJsonOptions('DELETE'));
    },
  }
};
