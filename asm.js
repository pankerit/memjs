const fs = require("fs");
const path = require("path");
const dllBase64 = require("./keystonedll.js");
const os = require("os");
const ref = require("ref-napi");
const ffi = require("ffi-napi");

// Create the keystone.dll
const dllPath = path.join(os.tmpdir(), "keystone.dll");
fs.writeFileSync(dllPath, Buffer.from(dllBase64, "base64"));

const ks_engine = "void";
const ks_enginePtr = ref.refType(ks_engine);
const ks_enginePtrPtr = ref.refType(ks_enginePtr);
const ks_arch = "int";
const ks_err = "int";
const ks_opt_type = "int";
const uintPtr = ref.refType("uint");
const ucharPtr = ref.refType("uchar");
const ucharPtrPtr = ref.refType(ucharPtr);
const size_tPtr = ref.refType("size_t");
const stringPtr = ref.refType("string");

// Load the keystone.dll
const Keystone = ffi.Library(dllPath, {
  ks_version: ["uint", [uintPtr, uintPtr]],
  ks_arch_supported: ["bool", [ks_arch]],
  ks_open: [ks_err, [ks_arch, "int", ks_enginePtrPtr]],
  ks_close: [ks_err, [ks_enginePtr]],
  ks_errno: ["int", [ks_enginePtr]],
  ks_strerror: ["string", [ks_err]],
  ks_option: [ks_err, [ks_enginePtr, ks_opt_type, "size_t"]],
  ks_asm: [
    "int",
    [ks_enginePtr, "string", "uint64", ucharPtrPtr, size_tPtr, size_tPtr],
  ],
  ks_free: ["void", ["pointer"]],
});

exports.ARCH_X86 = 4;
exports.MODE_16 = 2;
exports.MODE_32 = 4;
exports.MODE_64 = 8;

class Asm {
  constructor(arch, mode) {
    const _ks = ref.alloc(ks_enginePtr);
    const err = Keystone.ks_open(arch, mode, _ks);

    if (err !== 0) {
      this._ks = null;
      throw new Error("Error: failed on ks_open()");
    }
    this._ks = _ks.deref();
    this.setOption(1, 1);
  }

  load(code) {
    var encoding = ref.alloc("uchar *"),
      size = ref.alloc("size_t"),
      count = ref.alloc("size_t"),
      err,
      msg;

    if (Keystone.ks_asm(this._ks, code, 0, encoding, size, count) !== 0) {
      err = this.errno;
      msg = Keystone.ks_strerror(err);
      throw new KsError(msg, err, count.deref());
    }

    return Buffer.from(ref.reinterpret(encoding.deref(), size.deref(), 0));
  }

  setOption(type, value) {
    const err = Keystone.ks_option(this._ks, type, value);
    if (err !== 0) {
      throw new Error("Error: failed on ks_option()");
    }
  }

  close() {
    Keystone.ks_close(this._ks);
    this._ks = null;
  }
}

module.exports = Asm;

// const asm = new Asm(exports.ARCH_X86, exports.MODE_64);
