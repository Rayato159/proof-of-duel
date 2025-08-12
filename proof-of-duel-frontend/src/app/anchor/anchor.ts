import { clusterApiUrl, Connection, } from "@solana/web3.js";
import type { ProofOfDuelProgram } from "./idlType";
import { IdlAccounts, Program, } from "@coral-xyz/anchor";
import idl from "./idl.json";

export const programConnection = new Connection(clusterApiUrl("devnet"));

export const program = new Program(idl as ProofOfDuelProgram, {
    connection: programConnection,
}) as Program<ProofOfDuelProgram>;

export type PlayerData = IdlAccounts<ProofOfDuelProgram>["player"];