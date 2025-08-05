import { TitleBar } from "./titleBar";
import Wallet from "./wallet";

export default async function Home() {
  return (
    <>
      <Wallet />
      <TitleBar />
    </>
  );
}
