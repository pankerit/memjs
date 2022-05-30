import {
    alloc_memory,
    close_handle,
    get_modules,
    get_processes,
    open_process,
    read_memory,
    sig_scan,
    write_memory,
} from './core'
import { Module } from './types'

class Process {
    public id: number
    public name: string
    public handle: number
    public path: string

    constructor(processName: string) {
        const process = open_process(processName)
        this.id = process.id
        this.name = process.name
        this.handle = process.handle
        this.path = process.path
    }

    close() {
        close_handle(this.handle)
    }

    getModules() {
        return get_modules(this.handle)
    }

    findModule(moduleName: string): Module | undefined {
        const modules = this.getModules()
        for (const module of modules) {
            if (module.name === moduleName) {
                return module
            }
        }
        return undefined
    }

    sigScan(signature: string, baseAddress: number) {
        return sig_scan(this.handle, signature, baseAddress)
    }

    readMemoryDWORD(address: number) {
        return this.readMemoryBuffer(address, 4).readUint32LE(0)
    }

    readMemoryFloat32(address: number) {
        return this.readMemoryBuffer(address, 4).readFloatLE(0)
    }

    readMemoryFloat64(address: number) {
        return this.readMemoryBuffer(address, 8).readDoubleLE(0)
    }

    readMemoryInt32(address: number) {
        return this.readMemoryBuffer(address, 4).readInt32LE(0)
    }

    readMemoryBuffer(address: number, size: number) {
        return read_memory(this.handle, address, size)
    }

    writeMemoryDWORD(address: number, value: number) {
        return this.writeMemoryBuffer(address, Process.int32ToBuffer(value))
    }

    writeMemoryInt32(address: number, value: number) {
        this.writeMemoryBuffer(address, Process.int32ToBuffer(value))
    }

    writeMemoryFloat32(address: number, value: number) {
        this.writeMemoryBuffer(address, Process.float32ToBuffer(value))
    }

    writeMemoryFloat64(address: number, value: number) {
        this.writeMemoryBuffer(address, Process.float64ToBuffer(value))
    }

    writeMemoryBuffer(address: number, buffer: Buffer) {
        write_memory(this.handle, address, buffer)
    }

    allocMemory(size: number) {
        return alloc_memory(this.handle, size)
    }

    static int32ToBuffer(value: number) {
        const buffer = Buffer.alloc(4)
        buffer.writeInt32LE(value, 0)
        return buffer
    }

    static float32ToBuffer(value: number) {
        const buffer = Buffer.alloc(4)
        buffer.writeFloatLE(value, 0)
        return buffer
    }

    static float64ToBuffer(value: number) {
        const buffer = Buffer.alloc(8)
        buffer.writeDoubleLE(value, 0)
        return buffer
    }

    static stringToBuffer(value: string) {
        return Buffer.from(value, 'utf8')
    }
}

export default Process
