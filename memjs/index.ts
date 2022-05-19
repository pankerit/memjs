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

    readMemoryBuffer(address: number, size: number) {
        return read_memory(this.handle, address, size)
    }

    writeMemoryBuffer(address: number, buffer: Buffer) {
        write_memory(this.handle, address, buffer)
    }

    allocMemory(size: number) {
        return alloc_memory(this.handle, size)
    }
}

export default Process
