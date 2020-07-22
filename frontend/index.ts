import 'regenerator-runtime/runtime';

import axios from 'axios';
import config from './config';

interface RefreshTokenRequest {
  readonly name?: string;
  readonly email?: string;
  readonly password?: string;
  readonly refresh_token?: string;
}

interface RefreshTokenResponse {
  readonly access_token: string;
  readonly refresh_token: string;
}

async function refresh(request: RefreshTokenRequest): Promise<RefreshTokenResponse> {
  const response = await axios.post<RefreshTokenResponse>('/api/v1/tokens', request);
  return response.data;
}

window.addEventListener('message', async (e) => {
  if (config.allowedOrigins.indexOf(e.origin) === -1) {
    return;
  }

  if (!e.source || e.source instanceof MessagePort || e.source instanceof ServiceWorker) {
    return;
  }

  switch (e.data.command) {
    case 'get_token':
      e.source.postMessage(
        {
          id: e.data.id,
          command: 'set_token',
          access_token: localStorage['access_token'],
          refresh_token: localStorage['refresh_token'],
        },
        e.origin,
      );
      break;

    case 'refresh_token': {
      const { name, email, password, refresh_token } = e.data;
      const request = { name, email, password, refresh_token };

      try {
        const response = await refresh(request);
        localStorage['access_token'] = response.access_token;
        localStorage['refresh_token'] = response.refresh_token;

        e.source.postMessage(
          {
            id: e.data.id,
            command: 'set_token',
            access_token: response.access_token,
            refresh_token: response.refresh_token,
          },
          e.origin,
        );
      } catch (err) {
        e.source.postMessage(
          {
            id: e.data.id,
            command: 'error',
            error: err.message,
          },
          e.origin,
        );
      }
      break;
    }
  }
});

if (window.parent) {
  window.parent.postMessage('sso_loaded', '*');
}
