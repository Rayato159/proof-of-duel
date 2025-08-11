import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProofOfDuelProgram } from "../target/types/proof_of_duel_program";
import { assert } from "chai";

describe("proof-of-duel-program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.proofOfDuelProgram as Program<ProofOfDuelProgram>;

  const player = anchor.web3.Keypair.generate();

  const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("player"), player.publicKey.toBuffer()],
    program.programId,
  );

  it("initialize player test", async () => {
    await airdrop(player.publicKey);

    await program.methods
      .initializePlayer()
      .accountsPartial({
        signer: player.publicKey,
        player: playerPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([player])
      .rpc();

    console.log("Player initialized");

    const playerAccount = await program.account.player.fetch(playerPda);

    assert.equal(playerAccount.win.toNumber(), 0);
    assert.equal(playerAccount.loss.toNumber(), 0);
  });

  it("increment win test", async () => {
    await program.methods
      .winIncrement()
      .accountsPartial({
        player: playerPda,
        wallet: player.publicKey,
      })
      .rpc();

    const playerAccount = await program.account.player.fetch(playerPda);

    assert.equal(playerAccount.win.toNumber(), 1);
  });

  it("increment loss test", async () => {
    await program.methods
      .lossIncrement()
      .accountsPartial({
        player: playerPda,
        wallet: player.publicKey,
      })
      .rpc();

    const playerAccount = await program.account.player.fetch(playerPda);

    assert.equal(playerAccount.loss.toNumber(), 1);
  });

  const airdrop = async (pubkey: anchor.web3.PublicKey) => {
    const sig = await anchor.getProvider().connection.requestAirdrop(pubkey, 0.01 * anchor.web3.LAMPORTS_PER_SOL);
    const blockhash = await anchor.getProvider().connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction({
      blockhash: blockhash.blockhash,
      lastValidBlockHeight: blockhash.lastValidBlockHeight,
      signature: sig,
    });
  };
});
