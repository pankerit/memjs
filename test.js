const mem = require('./dist/index')
console.log(mem)
const main = async () => {
    const process = new mem.default('osu!.exe')
    console.time('mem')
    const address = await process.sigScan(
        '8D 65 F4 5B 5E 5F 5D C3 00 00 00 00 00 00 2C DF CF 14 00 00 00 00 24 DF CF 14',
        0,
    )
    console.timeEnd('mem')
    console.log(address.toString(16))
}

main()
