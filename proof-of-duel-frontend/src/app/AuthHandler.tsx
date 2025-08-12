"use client";

import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useUser } from "@civic/auth/react";
import { useEffect } from "react";
import { program } from "./anchor/anchor";
import * as anchor from "@coral-xyz/anchor";

const AuthHandler = () => {
  const { publicKey, sendTransaction } = useWallet();
  const { user } = useUser();
  const { connection } = useConnection();

  useEffect(() => {
    if (publicKey && user?.name) {
      console.log("Public Key:", publicKey.toString());
      console.log("User Info:", user.given_name);

      fetch("http://localhost:8080/login", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          public_key: publicKey?.toString(),
          username: user?.name || "",
        }),
      })
        .then(async (response) => {
          if (response.ok) {
            console.log("Login successful");

            if (!publicKey) {
              console.error("Wallet not connected");
              return;
            }

            const [playerPda] = anchor.web3.PublicKey.findProgramAddressSync(
              [Buffer.from("player"), publicKey.toBuffer()],
              program.programId
            );

            // Check if player data already exists
            let playerData = await program.account.player.fetch(playerPda);
            if (playerData) {
              console.log("Player data already exists:", playerData);
              return;
            }

            console.log("Player PDA:", playerPda.toString());

            try {
              const transaction = await program.methods
                .initializePlayer()
                .accountsPartial({
                  player: playerPda,
                  signer: publicKey,
                  systemProgram: anchor.web3.SystemProgram.programId,
                })
                .transaction();

              const transactionSignature = await sendTransaction(
                transaction,
                connection
              );

              console.log(
                `View on explorer: https://solana.fm/tx/${transactionSignature}?cluster=devnet`
              );
            } catch (error) {
              console.error("Error:", error);
            }
          } else {
            console.error("Request failed", response);
          }
        })
        .catch((error) => {
          console.error("Error making request:", error);
        });
    }
  }, [publicKey, user?.name]);

  return null;
};

export default AuthHandler;
