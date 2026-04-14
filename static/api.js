import { DEFAULT_API_TIMEOUT_MS, STATS_API_TIMEOUT_MS } from './constants.js';

const API_VERSION = 'v1';
const buildUrl = (path) => `/${API_VERSION}${path}`;

// Request deduplication cache
const pendingRequests = new Map();

const request = async (url, options, timeout = DEFAULT_API_TIMEOUT_MS) => {
  // Deduplicate only GET requests — mutations must never be collapsed
  if (options.method === 'GET') {
    const requestKey = `GET:${url}`;
    if (pendingRequests.has(requestKey)) {
      return pendingRequests.get(requestKey);
    }

    const promise = doFetch(url, options, timeout, requestKey);
    pendingRequests.set(requestKey, promise);
    return promise;
  }

  return doFetch(url, options, timeout, null);
};

const doFetch = async (url, options, timeout, requestKey) => {
  const controller = new AbortController();
  const timeoutId = setTimeout(() => controller.abort(), timeout);

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
    if (requestKey) {
      pendingRequests.delete(requestKey);
    }
  }
};

const createJsonOptions = (method, body = null) => ({
  method,
  ...(body && {
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  }),
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

    getStats: async (locationId, gameId) => {
      const url = buildUrl(`/locations/${locationId}/games/${gameId}/reservations`);
      return request(url, createJsonOptions('GET'), STATS_API_TIMEOUT_MS);
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
