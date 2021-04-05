/*
 * index.ts
 * 
 */

import "./css/styles.css";

declare global {
    interface Window {
        mediasoup: any,
    }
}

import * as mediasoupClient from 'mediasoup-client';
window.mediasoup = mediasoupClient;

(async () => {
  // Note: files in `./pkg/` will be created on the first build.
  await import("./pkg/index");
})();
