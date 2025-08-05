"use client";

import { useConnection, useWallet } from "@solana/wallet-adapter-react";
import { useState } from "react";

const useBalance = () => {
  const [balance, setBalance] = useState<number>();
  // The Solana Wallet Adapter hooks
  const { connection } = useConnection();
  const { publicKey } = useWallet();

  if (publicKey) {
    connection.getBalance(publicKey).then(setBalance);
  }

  return balance;
};

export default function Wallet() {
  const balance = useBalance();
  const { publicKey } = useWallet();

  return (
    <>
      {publicKey && (
        <div className="font-bold text-white">
          <span>
            Balance: {balance ? `${balance / 1e9} SOL` : "Loading..."}
          </span>
        </div>
      )}
    </>
  );
}
