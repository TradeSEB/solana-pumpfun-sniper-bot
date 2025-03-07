import * as token from "@solana/spl-token";
import * as web3 from "@solana/web3.js";
import getBondingCurvePDA from "./getBondingCurvePDA";
import tokenDataFromBondingCurveTokenAccBuffer from "./tokenDataFromBondingCurveTokenAccBuffer";
import getBuyPrice from "./getBuyPrice";
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import { PumpFun } from "../idl/pump-fun";
import IDL from "../idl/pump-fun.json";
import getBondingCurveTokenAccountWithRetry from "./getBondingCurveTokenAccountWithRetry";
import { Connection, SystemProgram, TransactionMessage } from "@solana/web3.js";
import { executeJitoTx } from "../../utils/jito";
import dotenv from 'dotenv'
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import Client from "@triton-one/yellowstone-grpc";

dotenv.config()

const BOANDING_CURVE_ACC_RETRY_AMOUNT = 50;
const BOANDING_CURVE_ACC_RETRY_DELAY = 50;

const solanaConnection = new Connection(process.env.RPC_ENDPOINT!, 'processed');
const stakeConnection = new Connection(process.env.SEND_RPC_ENDPOINT!, 'processed')

// Constants
const ENDPOINT = process.env.GRPC_ENDPOINT!;
const TOKEN = process.env.GRPC_TOKEN!;


const client = new Client(ENDPOINT, TOKEN, {});

const keypair = web3.Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY!))

// Load Pumpfun provider
const provider = new AnchorProvider(solanaConnection, new Wallet(keypair), {
  commitment: "processed",
});
const program = new Program<PumpFun>(IDL as PumpFun, provider);

const programId = new web3.PublicKey('6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P');

// Create transaction

interface Payload {
  transaction: TransactionMessages;
}

interface TransactionMessages {
  content: string;
}

async function buyToken(
  mint: web3.PublicKey,
  connection: web3.Connection,
  keypair: web3.Keypair,
  solAmount: number,
  slippage: number,
  priorityFee?: number
) {
  try {
    let transaction = new web3.Transaction();

    console.log("--------------- Start Buy Token --------------", '\n')
    // Get/Create token account
    const associatedUser = await token.getAssociatedTokenAddress(mint, keypair.publicKey, false);

    transaction.add(
      token.createAssociatedTokenAccountInstruction(keypair.publicKey, associatedUser, keypair.publicKey, mint)
    );

    // Contact with t.me/SavantCat

  } catch (error) {
    console.error(error);
    return false
  }
}


export default buyToken;
