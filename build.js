const fs = require('fs-extra')
const path = require('path')
const pkg = require('./package.json')
const { execSync } = require('child_process')

try {
    fs.rmSync('./dist', { recursive: true })
} catch (e) {}

execSync('npm run build-release')
execSync('yarn tsc')

fs.copyFileSync('./README.md', './dist/README.md')
fs.copyFileSync('./index.node', './dist/index.node')
fs.copyFileSync('./lib/core.js', './dist/core.js')
fs.copyFileSync('./lib/core.d.ts', './dist/core.d.ts')
fs.copyFileSync('./lib/types.d.ts', './dist/types.d.ts')

delete pkg.devDependencies
delete pkg.scripts

fs.writeFileSync('./dist/package.json', JSON.stringify(pkg, null, 4))
