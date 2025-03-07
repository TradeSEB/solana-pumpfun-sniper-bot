import { ComputeBudgetProgram, Connection, Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction, VersionedTransaction, sendAndConfirmTransaction } from "@solana/web3.js"
import { TOKEN_PROGRAM_ID, createAssociatedTokenAccountIdempotentInstruction, createCloseAccountInstruction, createTransferCheckedInstruction, getAssociatedTokenAddress } from "@solana/spl-token";
import { SPL_ACCOUNT_LAYOUT, TokenAccount } from "@raydium-io/raydium-sdk";
import base58 from "bs58"
import dotenv from 'dotenv';

import { sleep } from "./utils/commonFunc";

dotenv.config()

const RPC_ENDPOINT = process.env.RPC_ENDPOINT;
const RPC_WEBSOCKET_ENDPOINT = process.env.WEBSOCKET_RPC_ENDPOINT;
const PRIVATE_KEY = process.env.PRIVATE_KEY;
const GATHER_SLIPPAGE = Number(process.env.GATHER_SLIPPAGE);
const GATHER_FEE_LEVEL = Number(process.env.GATHER_FEE_LEVEL);

const connection = new Connection(RPC_ENDPOINT!, { wsEndpoint: RPC_WEBSOCKET_ENDPOINT, commitment: "confirmed" });
const mainKp = Keypair.fromSecretKey(base58.decode(PRIVATE_KEY!))

const main = async () => {

    try {
        // main functionality here, get in touch with t.me/SavantCat


    } catch (error) {
        console.log("transaction error while processing", error)
        return
    }
}


const getSellTxWithJupiter = async (wallet: Keypair, baseMint: PublicKey, amount: string) => {
    try {
        // Get sell transactions, contact with t.me/SavantCat
        return transaction
    } catch (error) {
        console.log("Failed to get sell transaction")
        return null
    }
};



interface Blockhash {
    blockhash: string;
    lastValidBlockHeight: number;
}

export const execute = async (transaction: VersionedTransaction, latestBlockhash: Blockhash, isBuy: boolean | 1 = true) => {

    const signature = await connection.sendRawTransaction(transaction.serialize(), { skipPreflight: true })
    const confirmation = await connection.confirmTransaction(
        {
            signature,
            lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
            blockhash: latestBlockhash.blockhash,
        }
    );

    if (confirmation.value.err) {
        console.log("Confirmation error")
        return ""
    } else {
        if (isBuy === 1) {
            return signature
        } else if (isBuy)
            console.log(`Success in buy transaction: https://solscan.io/tx/${signature}`)
        else
            console.log(`Success in Sell transaction: https://solscan.io/tx/${signature}`)
    }
    return signature
}


main()