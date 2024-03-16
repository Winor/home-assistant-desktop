import {
    getAuth,
    createConnection,
    subscribeEntities,
    ERR_HASS_HOST_REQUIRED,
    Auth,
  } from "home-assistant-js-websocket";

  export async function getToken(hass: string, ) {
    let auth: Promise<Auth> = getAuth({hassUrl: hass, redirectUrl: `${window.location.toString()}/callback`})
  }