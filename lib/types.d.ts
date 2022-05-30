declare module 'mem-tool' {
    export interface Process {
        id: number
        name: string
        handle: number
        path: string
    }

    export interface Process_ {
        id: number
        name: string
    }

    export interface Module {
        base_address: number
        size: number
        name: string
        path: string
    }
}
