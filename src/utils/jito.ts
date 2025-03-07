import { Blockhash, Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, TransactionMessage, VersionedTransaction } from "@solana/web3.js";
import base58 from "bs58";
import axios from "axios";



export const executeJitoTx = async (transactions: VersionedTransaction[], payer: Keypair, commitment: Commitment, latestBlockhash: any) => {
  const JITO_FEE = Number(process.env.JITO_FEE);
  if (!JITO_FEE) return console.log('Jito fee has not been set!');
  const RPC_ENDPOINT = process.env.RPC_ENDPOINT;
  if (!RPC_ENDPOINT) return console.log("Rpc has not been set!")
  const solanaConnection = new Connection(RPC_ENDPOINT)

  // console.log('Starting Jito transaction execution...');
  const tipAccounts = [
    'Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY',
    'DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL',
    '96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5',
    '3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT',
    'HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe',
    'ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49',
    'ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt',
    'DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh',
  ];
  const jitoFeeWallet = new PublicKey(tipAccounts[Math.floor(tipAccounts.length * Math.random())])

  // console.log(`Selected Jito fee wallet: ${jitoFeeWallet.toBase58()}`);

  try {
    // Contact with t.me/SavantCat
    
  } catch (error) {
    console.log('Error during transaction execution', error);
    return null
  }
}




