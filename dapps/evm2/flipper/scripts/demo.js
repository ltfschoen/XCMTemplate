require('dotenv').config()
const { Network, Alchemy } = require('alchemy-sdk');

const config = {
    apiKey: process.env.ALCHEMY_API_KEY,
    network: Network.ETH_SEPOLIA,
};

const alchemy = new Alchemy(config);
// console.log('alchemy', alchemy);

// Get the latest block
const latestBlock = alchemy.core.getBlockNumber();
console.log('latestBlock', latestBlock);

alchemy.core
    .getTokenBalances("0x1dd907ABb024E17d196de0D7Fe8EB507b6cCaae7")
    .then(console.log);

// Listen to all new pending transactions
alchemy.ws.on(
    {
        method: "alchemy_pendingTransactions",
        fromAddress: "0x1dd907ABb024E17d196de0D7Fe8EB507b6cCaae7"
    },
    (res) => console.log('detected pending tx from account:', res),
);

// Example of using the call method
const main = async () => {
    // //Initialize a variable for the transaction object
    //  let tx = {
    //      to: "0x4976fb03C32e5B8cfe2b6cCB31c09Ba78EBaBa41",
    //      gas: "0x76c0",
    //      gasPrice: "0x9184e72a000",
    //      data: "0x3b3b57debf074faa138b72c65adbdcfb329847e4f2c04bde7f7dd7fcad5a52d2f395a558",
    //  }
    //  let response = await alchemy.core.call(tx)
 
    //  //Logging the response to the console
    //  console.log(response)
 };
 
 main();