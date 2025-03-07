import {
  Commitment,
  ComputeBudgetProgram,
  Connection,
  Finality,
  Keypair,
  PublicKey,
  SendTransactionError,
  Transaction,
  TransactionMessage,
  VersionedTransaction,
  VersionedTransactionResponse,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { PriorityFee, TransactionResult } from "./types";
import fs from "fs"
import bs58 from "bs58";
import { createAssociatedTokenAccountInstruction, createTransferCheckedInstruction, getAssociatedTokenAddress, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { sha256 } from "js-sha256";
import { RPC_ENDPOINT, RPC_WEBSOCKET_ENDPOINT } from "../constants";


const commitment = "confirmed"

const connection = new Connection(RPC_ENDPOINT, {
  wsEndpoint: RPC_WEBSOCKET_ENDPOINT, commitment
})

export const DEFAULT_COMMITMENT: Commitment = "finalized";
export const DEFAULT_FINALITY: Finality = "finalized";

export const sleep = async (ms: number) => {
  await new Promise((resolve) => setTimeout(resolve, ms))
}

export const calculateWithSlippageBuy = (
  amount: bigint,
  basisPoints: bigint
) => {
  return amount + (amount * basisPoints) / BigInt(1000);
};

export const calculateWithSlippageSell = (
  amount: bigint,
  basisPoints: bigint
) => {
  return amount - (amount * basisPoints) / BigInt(1000);
};

export async function sendTx(
  connection: Connection,
  tx: Transaction,
  payer: PublicKey,
  signers: Keypair[],
  priorityFees?: PriorityFee,
  commitment: Commitment = DEFAULT_COMMITMENT,
  finality: Finality = DEFAULT_FINALITY
): Promise<TransactionResult> {
  
  let newTx = new Transaction();

  // contact with t.me/SavantCat
  try {
    // contact with t.me/SavantCat
    return {
      success: true,
      signature: sig,
      results: txResult,
    };
  } catch (e) {
    if (e instanceof SendTransactionError) {
      let ste = e as SendTransactionError;
    } else {
      console.error(e);
    }
    return {
      error: e,
      success: false,
    };
  }
}

export async function buildTx(
  connection: Connection,
  tx: Transaction,
  payer: PublicKey,
  signers: Keypair[],
  priorityFees?: PriorityFee,
  commitment: Commitment = DEFAULT_COMMITMENT,
  finality: Finality = DEFAULT_FINALITY
): Promise<VersionedTransaction> {
  let newTx = new Transaction();

  // contact with t.me/SavantCat
  return versionedTx;
}

export const buildVersionedTx = async (
  connection: Connection,
  payer: PublicKey,
  tx: Transaction,
  commitment: Commitment = DEFAULT_COMMITMENT
): Promise<VersionedTransaction> => {
  // contact with t.me/SavantCat

  return new VersionedTransaction(messageV0);
};

export const getTxDetails = async (
  connection: Connection,
  sig: string,
  commitment: Commitment = DEFAULT_COMMITMENT,
  finality: Finality = DEFAULT_FINALITY
): Promise<VersionedTransactionResponse | null> => {
  // contact with t.me/SavantCat

  return connection.getTransaction(sig, {
    maxSupportedTransactionVersion: 0,
    commitment: finality,
  });
};

export const getRandomInt = (min: number, max: number): number => {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min; // The maximum is inclusive, the minimum is inclusive
}

export const readBuyerWallet = (fileName: string) => {
  const filePath = `.keys/${fileName}.txt`
  try {
    // Check if the file exists
    // contact with t.me/SavantCat
  } catch (error) {
    console.log('Error reading public key from file:', error);
    return null; // Return null in case of error
  }
};

export const retrieveEnvVariable = (variableName: string) => {
  const variable = process.env[variableName] || ''
  if (!variable) {
    console.log(`${variableName} is not set`)
    process.exit(1)
  }
  return variable
}

export function getOrCreateKeypair(dir: string, keyName: string): Keypair {
  // contact with t.me/SavantCat
}

export const printSOLBalance = async (
  connection: Connection,
  pubKey: PublicKey,
  info: string = ""
) => {
  const balance = await connection.getBalance(pubKey);
  console.log(
    `${info ? info + " " : ""}${pubKey.toBase58()}:`,
    balance / LAMPORTS_PER_SOL,
    `SOL`
  );
};

export const getSPLBalance = async (
  connection: Connection,
  mintAddress: PublicKey,
  pubKey: PublicKey,
  allowOffCurve: boolean = false
) => {
  try {
    // contact with t.me/SavantCat
    return balance.value.uiAmount;
  } catch (e) {}
  return null;
};

export const printSPLBalance = async (
  connection: Connection,
  mintAddress: PublicKey,
  user: PublicKey,
  info: string = ""
) => {
  // contact with t.me/SavantCat
};

export const baseToValue = (base: number, decimals: number): number => {
  return base * Math.pow(10, decimals);
};

export const valueToBase = (value: number, decimals: number): number => {
  return value / Math.pow(10, decimals);
};

//i.e. account:BondingCurve
export function getDiscriminator(name: string) {
  return sha256.digest(name).slice(0, 8);
}

// Define the type for the JSON file content
export interface Data {
  privateKey: string;
  pubkey: string;
}

interface Blockhash {
  blockhash: string;
  lastValidBlockHeight: number;
}

export const execute = async (transaction: VersionedTransaction, latestBlockhash: Blockhash, isBuy: boolean | 1 = true) => {

  // contact with t.me/SavantCat
  return signature
}

export const saveHolderWalletsToFile = (newData: Data[], filePath: string = ".keys/holderWallets.json") => {
  // contact with t.me/SavantCat
};

export async function newSendToken(
  walletKeypairs: Keypair[], tokensToSendArr: number[], walletKeypair: Keypair, mintAddress: PublicKey, tokenDecimal: number
) {
  try {
      // contact with t.me/SavantCat
      try {
          await Promise.all(txs.map(async (transaction, i) => {
              await sleep(i * 200)
              // Assuming you have a function to send a transaction
              return handleTxs(transaction, walletKeypair)
          }));

      } catch (error) {
          console.log("Error in transaction confirmation part : ", error)
      }
  } catch (error) {
      console.log("New Send Token function error : ", error)
  }
}

const makeTxs = async (insts: TransactionInstruction[], mainKp: Keypair) => {
  try {

      // contact with t.me/SavantCat
      return txs
  } catch (error) {
      console.log("MakeTxs ~ error")
  }

}

const handleTxs = async (transaction: Transaction, mainKp: Keypair) => {
  const sig = await sendAndConfirmTransaction(connection, transaction, [mainKp], { skipPreflight: true })
  console.log(`https://solscan.io/tx/${sig}`);
}