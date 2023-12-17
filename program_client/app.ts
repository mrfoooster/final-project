import {
    Connection,
    PublicKey,
    Keypair,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,
  } from '@solana/web3.js';
  
  async function vote(connection: Connection, payer: Keypair, votingContract: PublicKey) {
    // Replace the following line with your smart contract's specific vote method invocation
    const transaction = new Transaction().add(
      // Example: Call a vote method on your smart contract
      // (replace the following line with your actual method invocation)
      SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: votingContract,
        lamports: 1000000, // Sending 1 SOL for testing
      })
    );
  
    // Sign and send the transaction
    await sendAndConfirmTransaction(connection, transaction, [payer]);
  }
  
  async function getResults(connection: Connection, votingContract: PublicKey) {
    // Replace the following line with your smart contract's specific getResults method invocation
    // Example: Call a getResults method on your smart contract
    const result = await connection.getBalance(votingContract);
  
    console.log('Voting results:', result);
  }
  
  async function main() {

    // Load the secret key from your generated file or provide it raw
    const secretKeyString = "B2m7XibJgJySKcNANNYAb9WhL5y7HRjzrxNhJ7jtvayS";
    const secretKeyBytes = Uint8Array.from(Buffer.from(secretKeyString, 'base64'));

    // Connect to the Solana cluster
    const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
  
    // Create a wallet (payer)
    const payer = Keypair.fromSecretKey(secretKeyBytes);
  
    // Replace the following line with your smart contract's address
    const votingContract = new PublicKey('Ao6cuPkcvg2cJgZpth5giqKjfDySGMrtcCpCcvU4F9nJ');
  
    // Perform a vote
    await vote(connection, payer, votingContract);
  
    // Get and log voting results
    await getResults(connection, votingContract);

    

  }
  
  main().catch((error) => console.error(error));
  