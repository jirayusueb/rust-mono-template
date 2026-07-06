import { Header } from "./header";
import type { ReactNode } from "react";

export function AppShell({ children }: { children: ReactNode }) {
  return (
    <div className="flex min-h-svh flex-col">
      <Header />
      <main className="flex flex-1 items-start justify-center px-6 py-12 lg:py-16">{children}</main>
    </div>
  );
}
