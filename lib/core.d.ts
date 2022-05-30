import type { Process, Process_, Module } from 'mem-tool'

/**
 * @param {string} processName - The name of the process to open.
 * @returns {Process} - The opened process.
 */
export declare const open_process: (processName: string) => Process

/**
 * @param {number} handle - The handle of the process you want to close.
 */
export declare const close_handle: (handle: number) => void

/**
 * @param {number} handle - The handle to the process.
 * @returns {Module[]} - An array of modules in the process.
 */
export declare const get_modules: (handle: number) => Module[]

/**
 * @returns {Process_} The process information.
 */
export declare const get_processes: () => Process_[]

/**
 * @param {number} handle Process handle
 * @param {string} signature Signature to search for
 * @param {number} baseAddress Base address to start searching from
 * @returns {Promise<number>} Address of the first occurrence of the signature
 */
export declare const sig_scan: (handle: number, signature: string, baseAddress: number) => Promise<number | undefined>

/**
 * @param {number} handle Process handle
 * @param {number} address Address to read from
 * @param {number} size Size of the buffer to read
 * @returns {Buffer} Buffer
 */
export declare const read_memory: (handle: number, address: number, size: number) => Buffer

/**
 * @param {number} handle Process handle
 * @param {number} address Address to write to
 * @param {Buffer} buffer Buffer to write
 */
export declare const write_memory: (handle: number, address: number, buffer: Buffer) => void

/**
 * @param {number} handle Process handle
 * @param {number} size Size of the buffer
 * @returns {Buffer} Buffer of the read memory
 */
export declare const alloc_memory: (handle: number, size: number) => number
