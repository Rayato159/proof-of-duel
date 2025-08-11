"use client";

import {
  WalletModalProvider,
  WalletMultiButton,
} from "@solana/wallet-adapter-react-ui";
import { CivicAuthProvider } from "@civic/auth-web3/nextjs";
import {
  ConnectionProvider,
  WalletProvider,
} from "@solana/wallet-adapter-react";

import AuthHandler from "./AuthHandler";

interface ProvidersProps {
  children: React.ReactNode;
}

export default function Providers({ children }: ProvidersProps) {
  const endpoint = "https://api.devnet.solana.com";

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider wallets={[]} autoConnect>
        <WalletModalProvider>
          <CivicAuthProvider>
            <WalletMultiButton />
            <AuthHandler />
            {children}
          </CivicAuthProvider>
        </WalletModalProvider>
      </WalletProvider>
    </ConnectionProvider>
  );
}
