/// <reference types="node" />

export interface Process {
    id: number
    name: string
    handle: number
}

export interface Module {
    baseAddress: number
    size: number
    name: string
    path: string
}

export declare const open_process: (processName: string) => Process

export declare const close_handle: (handle: number) => boolean

export declare const sig_scan_sync: (handle: number, signature: string, baseAddress: number) => number | undefined

export declare const sig_scan: (handle: number, signature: string, baseAddress: number) => Promise<number | undefined>

export declare const sig_scan_module_sync: (
    handle: number,
    processId: number,
    signature: string,
    moduleName: string,
) => number | undefine

export declare const sig_scan_module: (
    handle: number,
    processId: number,
    signature: string,
    moduleName: string,
) => Promise<number | undefine>

export declare const read_memory_buffer: (handle: number, address: number, size: number) => Buffer

export declare const write_memory_buffer: (handle: number, address: number, buffer: Buffer) => void

export declare const alloc_memory: (handle: number, size: number) => number

export declare const read_memory_u32: (handle: number, address: number) => number

export declare const write_memory_u32: (handle: number, address: number, value: number) => void

export declare const read_memory_u64: (handle: number, address: number) => number

export declare const write_memory_u64: (handle: number, address: number, value: number) => void

export declare const read_memory_i32: (handle: number, address: number) => number

export declare const write_memory_i32: (handle: number, address: number, value: number) => void

export declare const read_memory_i64: (handle: number, address: number) => number

export declare const write_memory_i64: (handle: number, address: number, value: number) => void

export declare const read_memory_f32: (handle: number, address: number) => number

export declare const write_memory_f32: (handle: number, address: number, value: number) => void

export declare const read_memory_f64: (handle: number, address: number) => number

export declare const write_memory_f64: (handle: number, address: number, value: number) => void

export declare const read_memory_bool: (handle: number, address: number) => boolean

export declare const write_memory_bool: (handle: number, address: number, value: boolean) => void

export declare const read_memory_string: (handle: number, address: number, size: number) => string

export declare const write_memory_string: (handle: number, address: number, value: string) => void

export declare const get_process_path: (handle: number) => string

export declare const get_process_modules: (processId: number) => Module[]
