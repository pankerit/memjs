// import { Buffer } from "node:buffer";

const memjs = require("./index.node");

class Process {
  constructor(processName) {
    this.boxed = memjs.open_process(processName);
  }

  sigScan(signature, baseAddress = 0) {
    const result = memjs.sig_scan(this.boxed, signature, baseAddress);
    return result;
  }

  writeMemory(address, value) {
    memjs.write_memory(this.boxed, address, Buffer.from(value));
  }

  readMemory(address, size) {
    memjs.read_memory(this.boxed, address, size);
  }
}

const process = new Process("osu!.exe");

const main = async () => {
  const addr = await process.sigScan("64 00 00 00 00 00 00 ? 38 5B EC 00");
  console.log(addr.toString(16));
  const addr2 = await process.sigScan("CD AD B0 71 B0 67", addr);
  console.log(addr2.toString(16));
};

main();
for (let i = 0; i < 100; i++) {}
