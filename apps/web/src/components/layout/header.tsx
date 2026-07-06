"use client";

import { useHeaderSession } from "./hooks/use-header-session";
import { HeaderView } from "./header-view";

export function Header() {
  const { session, isLoading, signOut } = useHeaderSession();
  return <HeaderView session={session} isLoading={isLoading} onSignOut={signOut} />;
}
