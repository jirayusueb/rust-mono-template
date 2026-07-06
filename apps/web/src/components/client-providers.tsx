"use client";

import type { ReactNode } from "react";
import { NuqsAdapter } from "nuqs/adapters/react";

export function ClientProviders({ children }: { children: ReactNode }) {
  return <NuqsAdapter>{children}</NuqsAdapter>;
}
