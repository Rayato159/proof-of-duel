import { makeServerProgram } from '@/app/anchor/anchorServer';
import * as anchor from '@coral-xyz/anchor';

export async function POST(req: Request) {
    const program = makeServerProgram();

    const body = await req.json();
    const publicKey: string = body.public_key;


    const playerPublicKey = new anchor.web3.PublicKey(publicKey);

    const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("player"), playerPublicKey.toBuffer()],
        program.programId,
    );

    try {
        const tx = await program.methods
            .winIncrement()
            .accountsPartial({
                player: playerPda,
                wallet: playerPublicKey,
            })
            .rpc();

        console.log(`Transaction successful: https://solana.fm/tx/${tx}?cluster=devnet`);
    } catch (error) {
        console.error("Error incrementing win:", error);
        return new Response(null, {
            status: 500,
        });
    }

    return new Response(null, {
        status: 200,
        headers: { 'Content-Type': 'application/json' }
    })
}