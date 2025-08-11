import "./globals.css";
import Providers from "./Providers";

export const metadata = {
  title: "Proof of Duel",
  description: "Duel with your friends in a decentralized way",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body className="bg-black text-white font-sans">
        <div className="flex flex-col items-center justify-center h-screen gap-6">
          <Providers>{children}</Providers>
        </div>
      </body>
    </html>
  );
}
