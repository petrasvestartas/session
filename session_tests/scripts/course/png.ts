import fs from 'node:fs';
import {deflateSync} from 'node:zlib';

function crc(bytes: Uint8Array) {
    let value = 0xffffffff;
    for (const byte of bytes) {
        value ^= byte;
        for (let i = 0; i < 8; i++) value = (value >>> 1) ^ ((value & 1) ? 0xedb88320 : 0);
    }
    return (value ^ 0xffffffff) >>> 0;
}

function chunk(type: string, data: Buffer) {
    const tag = Buffer.from(type);
    const size = Buffer.alloc(4), checksum = Buffer.alloc(4);
    size.writeUInt32BE(data.length);
    checksum.writeUInt32BE(crc(Buffer.concat([tag, data])));
    return Buffer.concat([size, tag, data, checksum]);
}

export function ppm(file: string) {
    const bytes = fs.readFileSync(file);
    const header = bytes.subarray(0, 100).toString('ascii').match(/^P6\n(\d+) (\d+)\n255\n/);
    if (!header) throw Error(`Unsupported RGB8 PPM header: ${file}`);
    const width = Number(header[1]), height = Number(header[2]);
    const pixels = bytes.subarray(header[0].length);
    if (pixels.length !== width * height * 3) throw Error(`Incomplete PPM: ${file}`);
    return {width, height, pixels};
}

export function png(file: string): Buffer {
    const {width, height, pixels} = ppm(file);
    const header = Buffer.alloc(13);
    header.writeUInt32BE(width, 0);
    header.writeUInt32BE(height, 4);
    header[8] = 8;
    header[9] = 2;
    const rows = Buffer.alloc(height * (1 + width * 3));
    for (let y = 0; y < height; y++) pixels.copy(rows, y * (1 + width * 3) + 1, y * width * 3, (y + 1) * width * 3);
    return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]), chunk('IHDR', header), chunk('IDAT', deflateSync(rows)), chunk('IEND', Buffer.alloc(0))]);
}
