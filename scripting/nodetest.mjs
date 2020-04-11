import { Worker } from "worker_threads";

const THREAD_COUNT = 4;

for (let i = 0; i < THREAD_COUNT; i++) {
  new Worker("./connect.mjs", { argv: [`Spawned: ${i}`] });
}
