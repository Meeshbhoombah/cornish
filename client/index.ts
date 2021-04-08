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

import { types as mediasoupTypes } from 'mediasoup-client';
window.mediasoup.types = mediasoupTypes;

(async () => {
  // Note: files in `./pkg/` will be created on the first build.
  await import("./pkg/index");
})();
