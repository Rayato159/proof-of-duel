import 'server-only';
import * as anchor from '@coral-xyz/anchor';
import { Keypair, Transaction, VersionedTransaction } from '@solana/web3.js';
import bs58 from 'bs58';
import { programConnection } from '@/app/anchor/anchor';
import idl from '@/app/anchor/idl.json';
import { ProofOfDuelProgram } from './idlType';

function walletFromKeypair(kp: Keypair): anchor.Wallet {
    const signTx = async <T extends Transaction | VersionedTransaction>(tx: T): Promise<T> => {
        if (tx instanceof VersionedTransaction) {
            tx.sign([kp]);
            return tx;
        } else {
            tx.partialSign(kp);
            return tx;
        }
    };

    const signAll = async <T extends Transaction | VersionedTransaction>(txs: T[]): Promise<T[]> => {
        for (const tx of txs) {
            if (tx instanceof VersionedTransaction) {
                tx.sign([kp]);
            } else {
                tx.partialSign(kp);
            }
        }
        return txs;
    };

    return {
        publicKey: kp.publicKey,
        payer: kp,
        signTransaction: signTx,
        signAllTransactions: signAll,
    };
}

export function makeServerProgram(): anchor.Program<ProofOfDuelProgram> {
    const pk = process.env.PRIVATE_KEY;
    if (!pk) throw new Error('PRIVATE_KEY is missing');
    const kp = Keypair.fromSecretKey(bs58.decode(pk));

    const provider = new anchor.AnchorProvider(
        programConnection,
        walletFromKeypair(kp),
        { commitment: 'confirmed' }
    );

    anchor.setProvider(provider);

    return new anchor.Program(idl as ProofOfDuelProgram, provider) as anchor.Program<ProofOfDuelProgram>;
}