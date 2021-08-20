const { 
    Connection, 
    Keypair, 
    Transaction, 
    sendAndConfirmTransaction, 
    PublicKey, 
    SystemProgram 
} = require('@solana/web3.js');
const { fs } = require('mz');

async function establishConnection() {
    // const rpcUrl = 'http://localhost:8899';
    const rpcUrl = 'https://api.devnet.solana.com';
    connection = new Connection(rpcUrl, 'confirmed');
    const version = await connection.getVersion();
    console.log('connection to cluster established:', rpcUrl, version);
}

async function createKeypairFromFile() {
    const secretKeyString = await fs.readFile('/home/knilesh/.config/solana/id.json', { encoding: 'utf8'});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}

async function createAccount() {

    const signer = await createKeypairFromFile();
    const newAccountPubkey = await PublicKey.createWithSeed(
        signer.publicKey,                                               
        "campaign2",                                                    
        new PublicKey("6JfaufYDEGtcWudhmdjVxKu36FsfRbQb31Nyk7m4d4hc"),  
    );

    const lamports = await connection.getMinimumBalanceForRentExemption(1024);
    const instruction = SystemProgram.createAccountWithSeed({
        fromPubkey: signer.publicKey,
        basePubkey: signer.publicKey,
        seed: "campaign2",
        newAccountPubkey,
        lamports, 
        space: 1024,
        programId : new PublicKey("6JfaufYDEGtcWudhmdjVxKu36FsfRbQb31Nyk7m4d4hc"),
    });
    const transaction = new Transaction().add(instruction);

    console.log(`address of campaign2 account: ${newAccountPubkey.toBase58()}`);

    await sendAndConfirmTransaction(connection, transaction, [signer]);
}

establishConnection();
createAccount();