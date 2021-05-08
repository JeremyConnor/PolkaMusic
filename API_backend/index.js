const express = require('express');
const cors = require('cors');
const polkadot = require('@polkadot/api');
const polka_util = require('@polkadot/util');
const Keyring  = require('@polkadot/keyring');
const polka_util_crypto =  require('@polkadot/util-crypto');
const config = require('./config/common.json');

console.log(config.CUSTOM_TYPES);

const port = 3030;


// create express app Initialization of object for express
const app = express();

// parse requests of content-type - application/json
app.use(cors());


app.get('/api/v1/', (req, res) => {
    res.json({message: 'Welcome to Polkamusic API service.'});
});

app.get('/api/v1/genesisHash', async(req,res) => {
       
    try {
        const wsProvider = new polkadot.WsProvider('ws://127.0.0.1:9944');
        const api = await polkadot.ApiPromise.create({ provider: wsProvider,types: config.CUSTOM_TYPES });
        console.log(api);

        // The length of an epoch (session) in Babe
        // let babeEpochDuration = api.consts.babe.epochDuration.toNumber();
      
        // // The amount required to create a new account
        // let balanceCreationFee = api.consts.balances.creationFee;

        // // // The amount required per byte on an extrinsic
        // // let = balancesTransactionByteFee = api.consts.balances.transactionByteFee.toNumber();

        // console.log(typeof(babeEpochDuration));
        res.send({GenesisHash: api.genesisHash
                            //  babeEpochDuration: babeEpochDuration
                });

    } catch(err) {
        res.status(500).json({error: err.toString()});
    }
})


/**
 * Rights Management pallet -
 */

 app.post('/api/v1/rightsManagementPallet', async(req,res) => {
       
    try {

        let account = new polkadot.Keyring({ type: 'sr25519' });
        let SrcId = req.body.SrcId;
        let song_id = req.body.song_id;

        await polka_util_crypto.cryptoWaitReady();
    
     
        // Initialise the provider to connect to the local node
        const provider = new polkadot.WsProvider('ws://127.0.0.1:9944');

        // Create the API and wait until ready
        const api = await polkadot.ApiPromise.create({ provider, types: config.CUSTOM_TYPES});


        // Type can be ed25519 or sr25519
        const keyring = new polkadot.Keyring({ type: 'sr25519' });
    
        // (Advanced, development-only) add with an implied dev seed and hard derivation
        const alice =keyring.addFromUri('//Alice')

        // const [entryHash, entrySize] = await Promise.all([
        //     api.query.system.account.hash(account),
        //     api.query.system.account.size(account)
        //     ]);
        console.log(`The current size is ${entrySize} bytes with a hash of ${entryHash}`);

        const rmp_reg =api.tx.rightsMgmtPortal.registerMusic(SrcId, song_id, account.address);
        const tx = await rmp_reg.signAndSend(alice,{ nonce: -1});
        console.log(`Submitted with hash ${tx}`)
       
        
        res.send({
                // currentSize:entrySize,
                // entryHash:entryHash,
                rmp:rmp_reg,
                Tx:tx,
                Msg:"Tx sent to Blockchain"
                });

    } catch(err) {
        res.status(500).json({error: err.toString()});
    }
})
















// Start the server
const server = app.listen(port, (error) => {
    if (error) {return console.log(`Error: ${error}`);}

    console.log(`Server listening on port ${server.address().port}`);
});

