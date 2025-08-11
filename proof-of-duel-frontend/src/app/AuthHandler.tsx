"use client";

import { useWallet } from "@solana/wallet-adapter-react";
import { useUser } from "@civic/auth/react";
import { useEffect } from "react";

const AuthHandler = () => {
  const { publicKey } = useWallet();
  const { user } = useUser();

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
