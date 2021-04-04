/*
 * index.ts
 * 
 */

import "./css/styles.css";

import * as mediasoup from 'mediasoup-client';
console.log(mediasoup);

(async () => {
  // Note: files in `./pkg/` will be created on the first build.
  await import("./pkg/index");
})();
