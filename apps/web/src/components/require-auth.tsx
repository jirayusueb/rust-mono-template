"use client";

import type { ReactNode } from "react";
import { useRequireAuth } from "@/components/layout/hooks/use-auth";

export function RequireAuth({ children }: { children: ReactNode }) {
  const session = useRequireAuth();
  if (session.isLoading || !session.data) return null;
  return <>{children}</>;
}
