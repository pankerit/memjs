const fs = require('fs-extra')
const path = require('path')
const pkg = require('./package.json')

fs.copyFileSync('./README.md', './dist/README.md')
fs.copyFileSync('./index.node', './dist/index.node')
fs.copyFileSync('./memjs/keystonedll.js', './dist/keystonedll.js')

delete pkg.devDependencies
delete pkg.scripts

fs.writeFileSync('./dist/package.json', JSON.stringify(pkg, null, 4))
