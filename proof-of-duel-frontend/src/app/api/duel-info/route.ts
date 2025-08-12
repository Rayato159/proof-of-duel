import { makeServerProgram } from "@/app/anchor/anchorServer";
import * as anchor from "@coral-xyz/anchor";

export async function POST(req: Request) {
    const program = makeServerProgram();

    const body = await req.json();
    const publicKey: string = body.public_key;

    const playerPublicKey = new anchor.web3.PublicKey(publicKey);

    const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("player"), playerPublicKey.toBuffer()],
        program.programId,
    );

    const playerAccount = await program.account.player.fetch(playerPda);
    const payload: { win: number, loss: number } = {
        win: playerAccount.win.toNumber(),
        loss: playerAccount.loss.toNumber(),
    };

    await fetch("http://localhost:8080/update-stats", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    })
        .then(async (response) => {
            if (response.ok) {
                console.log("Stats updated successfully");
            } else {
                console.error("Failed to update stats");
            }
        })
        .catch((error) => {
            console.error("Error making request:", error);
        });

    return new Response(JSON.stringify(payload), {
        status: 200,
        headers: { 'Content-Type': 'application/json' }
    })
}