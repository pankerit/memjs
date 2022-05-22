const { default: Mem } = require('./dist/index')
console.log(Mem)
const main = async () => {
    const process = new Mem('osu!.exe')
    const sig = '85 C0 74 06 0F B6 50 0C EB 02 33 D2 85 D2'

    console.time('mem')
    const address = await process.sigScan(sig, 0)
    console.timeEnd('mem')
    // console.log(address.toString(16))
    // console.log(address + sig.split(' ').length + 2)
    // console.log(Buffer.from([0xbc, 0x46, 0x1b, 0x04]).readInt32LE(0)) // 68896444 = 0x41b46bc
    // console.log(Mem.int32ToBuffer(100))
    // process.writeMemoryFloat64(0x0129d918, 100.00001)
    // console.log(process.readMemoryFloat64(0x0129d918))
    // console.log(Buffer.from(Buffer.from([0xc8, 0x36, 0x03, 0x04]).readInt32LE(0)))
    // let data = process.readMemoryBufferFromPointers([address + 44], 4)
    // console.log(data)
    // console.log('test')
}

main()
// C3 00 00 00 00 F8 EB 62 15 00 00 00 00 F0 EB 62 15 ? ? ? ? 57 56 8B F9 8B 35

setInterval(() => {}, 1000)
// 85 C0 74 06 0F B6 50 0C EB 02 33 D2 85 D2 0F 84 B7 00 00 00 80 3D 26 68 E0 02 00 0F 85 AA 00 00 00 8B 05
// const conv = (num) => [(num >> 24) & 255, (num >> 16) & 255, (num >> 8) & 255, num & 255]
// console.log(Buffer.from(conv(68896444).reverse()))
// console.log(Buffer.from(conv(68896444).reverse()).readInt32LE().toString(16))
