import { TitleBar } from "./TitleBar";
import Wallet from "./Wallet";

export default async function Home() {
  return (
    <>
      <Wallet />
      <TitleBar />
    </>
  );
}
