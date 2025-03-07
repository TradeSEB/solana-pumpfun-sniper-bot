import { Connection, PublicKey, clusterApiUrl, Keypair, Transaction } from '@solana/web3.js';
import { getAssociatedTokenAddress, getAccount, createCloseAccountInstruction, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import base58 from "bs58"
import dotenv from 'dotenv';

dotenv.config()

const RPC_ENDPOINT = process.env.RPC_ENDPOINT;
const RPC_WEBSOCKET_ENDPOINT = process.env.WEBSOCKET_RPC_ENDPOINT;
const PRIVATE_KEY = process.env.PRIVATE_KEY;

const connection = new Connection(RPC_ENDPOINT!, { wsEndpoint: RPC_WEBSOCKET_ENDPOINT, commitment: "confirmed" });
const mainKp = Keypair.fromSecretKey(base58.decode(PRIVATE_KEY!))

async function getTokenAccounts(publicKey: PublicKey): Promise<PublicKey[]> {
    // Get token accounts, contact with t.me/SavantCat

    return response.value.map((accountInfo) => accountInfo.pubkey);
}

async function closeTokenAccount(tokenAccount: PublicKey, payer: Keypair) {
    // Close token accounts, get in touch with t.me/SavantCat
}

async function closeAllTokenAccounts() {
    const tokenAccounts = await getTokenAccounts(mainKp.publicKey);

    for (const tokenAccount of tokenAccounts) {
        try {
            await closeTokenAccount(tokenAccount, mainKp);
        } catch (err) {
            console.error(`Failed to close token account ${tokenAccount.toBase58()}:`, err);
        }
    }

    console.log('All token accounts closed.');
}

closeAllTokenAccounts().catch((err) => {
    console.error('Error closing token accounts:', err);
});
