"use client";

import { useSession, useSignOut } from "./use-auth";

export function useHeaderSession() {
  const session = useSession();
  const signOut = useSignOut();
  return { session: session.data ?? undefined, isLoading: session.isLoading, signOut };
}
