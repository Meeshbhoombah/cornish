/*
 * index.ts
 * 
 */

import "./css/styles.css";

declare global {
    interface Window {
        mediasoup: any,
        mediasoupTypes: any,
    }
}

import * as mediasoupClient from 'mediasoup-client';
window.mediasoup = mediasoupClient;

import { types } from 'mediasoup-client';
window.mediasoupTypes = types;

(async () => {
  // Note: files in `./pkg/` will be created on the first build.
  await import("./pkg/index");
})();
