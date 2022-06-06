/// <reference types="node" />
import {
    alloc_memory,
    close_handle,
    get_process_modules,
    get_process_path,
    Module,
    open_process,
    read_memory_bool,
    read_memory_buffer,
    read_memory_f32,
    read_memory_f64,
    read_memory_i32,
    read_memory_i64,
    read_memory_string,
    read_memory_u32,
    read_memory_u64,
    sig_scan,
    sig_scan_module,
    sig_scan_module_sync,
    sig_scan_sync,
    write_memory_bool,
    write_memory_buffer,
    write_memory_f32,
    write_memory_f64,
    write_memory_i32,
    write_memory_i64,
    write_memory_string,
    write_memory_u32,
    write_memory_u64,
} from './core'

export class Process {
    public id: number
    public name: string
    public handle: number

    constructor(processName: string) {
        const process = open_process(processName)
        this.id = process.id
        this.name = process.name
        this.handle = process.handle
    }

    close(): boolean {
        return close_handle(this.handle)
    }

    sigScanSync(signature: string, baseAddress: number = 0): number | undefined {
        return sig_scan_sync(this.handle, signature, baseAddress)
    }

    sigScan(signature: string, baseAddress: number = 0): Promise<number | undefined> {
        return sig_scan(this.handle, signature, baseAddress)
    }

    sigScanModuleSync(signature: string, moduleName: string): number | undefined {
        return sig_scan_module_sync(this.handle, this.id, signature, moduleName)
    }

    sigScanModule(signature: string, moduleName: string): Promise<number | undefined> {
        return sig_scan_module(this.handle, this.id, signature, moduleName)
    }

    readMemoryBuffer(address: number, size: number): Buffer {
        return read_memory_buffer(this.handle, address, size)
    }

    writeMemoryBuffer(address: number, buffer: Buffer): void {
        write_memory_buffer(this.handle, address, buffer)
    }

    allocMemory(size: number): number {
        return alloc_memory(this.handle, size)
    }

    readMemoryU32(address: number): number {
        return read_memory_u32(this.handle, address)
    }

    writeMemoryU32(address: number, value: number): void {
        write_memory_u32(this.handle, address, value)
    }

    readMemoryU64(address: number): number {
        return read_memory_u64(this.handle, address)
    }

    writeMemoryU64(address: number, value: number): void {
        write_memory_u64(this.handle, address, value)
    }

    readMemoryI32(address: number): number {
        return read_memory_i32(this.handle, address)
    }

    writeMemoryI32(address: number, value: number): void {
        write_memory_i32(this.handle, address, value)
    }

    readMemoryI64(address: number): number {
        return read_memory_i64(this.handle, address)
    }

    writeMemoryI64(address: number, value: number): void {
        write_memory_i64(this.handle, address, value)
    }

    readMemoryF32(address: number): number {
        return read_memory_f32(this.handle, address)
    }

    writeMemoryF32(address: number, value: number): void {
        write_memory_f32(this.handle, address, value)
    }

    readMemoryF64(address: number): number {
        return read_memory_f64(this.handle, address)
    }

    writeMemoryF64(address: number, value: number): void {
        write_memory_f64(this.handle, address, value)
    }

    readMemoryBool(address: number): boolean {
        return read_memory_bool(this.handle, address)
    }

    writeMemoryBool(address: number, value: boolean): void {
        write_memory_bool(this.handle, address, value)
    }

    readMemoryString(address: number, size: number): string {
        return read_memory_string(this.handle, address, size)
    }

    writeMemoryString(address: number, value: string): void {
        write_memory_string(this.handle, address, value)
    }

    getProcessPath(): string {
        return get_process_path(this.handle)
    }

    getProcessModules(): Module[] {
        return get_process_modules(this.id)
    }
}
