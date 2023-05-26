const process = require('process');
const fs = require('fs');

// process.argv[0] is the path to the Node.js executable
// process.argv[1] is the path to the script file
// process.argv[2] is the first argument passed to the script
console.log("process.argv[2]: ", process.argv[2]);
console.log("process.argv[3]: ", process.argv[3]);

const abiFilePath = process.argv[2];
const jsonFilePath = process.argv[3];
const jsonContent = fs.readFileSync(jsonFilePath, 'utf8');
let jsonContentString = JSON.parse(jsonContent);
const prefix = "const abi = "
const middle = JSON.stringify(jsonContentString, null, 2);
const postfix = ";\nexport default abi;"
const combined = prefix.concat(middle, postfix);
// overwrite $PARENT_DIR/dapps/ink-rust/wasm-flipper/ui/components/abi.ts
// such that its `abi` variable contains the contents of the
// $PARENT_DIR/target/ink/flipper/flipper.json file
fs.writeFileSync(abiFilePath, combined);
console.log('replaced Flipper DApp ABI with new build JSON metadata');
