import { createConnection } from 'net';
import { Buffer } from 'buffer';
import worker from 'worker_threads';

const THREAD_DATA = `THREAD_DATA ${worker.threadId} ${process.argv}`;

function handle_connected() { }
function handle_response(buffer) {
    console.log(buffer.toString('utf-8'));
}

function get_bytes(string) {
    return Buffer.from(string, 'utf-8');
}

const client = createConnection(5002, '127.0.0.1', handle_connected);

client.on('data', handle_response);
client.write(get_bytes(THREAD_DATA));