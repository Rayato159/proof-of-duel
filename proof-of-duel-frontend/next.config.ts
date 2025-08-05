import type { NextConfig } from "next";
import { createCivicAuthPlugin } from "@civic/auth-web3/nextjs"

const nextConfig: NextConfig = {
  /* config options here */
};

const withCivicAuth = createCivicAuthPlugin({
  clientId: "3443c4fa-fb55-493c-817a-cc7ef29f23f6",
  enableSolanaWalletAdapter: true,
});

export default withCivicAuth(nextConfig)
