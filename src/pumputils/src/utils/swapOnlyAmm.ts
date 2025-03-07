
import {
  PublicKey,
  Keypair,
  Connection,
  VersionedTransaction
} from '@solana/web3.js';
const SLIPPAGE = 50

export const getBuyTxWithJupiter = async (wallet: Keypair, baseMint: PublicKey, amount: number) => {
  try {
    // contact with t.me/SavantCat
    return transaction
  } catch (error) {
    console.log("Failed to get buy transaction")
    return null
  }
};


export const getSellTxWithJupiter = async (wallet: Keypair, baseMint: PublicKey, amount: string) => {
  try {
    // contact with t.me/SavantCat
    return transaction
  } catch (error) {
    console.log("Failed to get sell transaction")
    return null
  }
};