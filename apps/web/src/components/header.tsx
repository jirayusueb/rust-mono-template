"use client";

import { Link } from "waku";
import { authClient, useInvalidateSession } from "../lib/auth-client";
import { Button } from "./ui/button";

export function Header() {
  const { data: session, isLoading } = authClient.useSession();
  const invalidate = useInvalidateSession();

  const handleLogout = async () => {
    await authClient.signOut();
    await invalidate();
  };

  return (
    <header className="flex items-center gap-4 p-6 lg:fixed lg:left-0 lg:top-0">
      <h2 className="text-lg font-bold tracking-tight">
        <Link to="/">Waku starter</Link>
      </h2>
      {!isLoading && session && (
        <div className="flex items-center gap-2 text-sm text-muted-foreground">
          <span>{session.user.email}</span>
          <Button variant="ghost" size="sm" onClick={handleLogout}>
            Sign out
          </Button>
        </div>
      )}
      {!isLoading && !session && (
        <div className="flex items-center gap-2 text-sm">
          <Link to="/login" className="text-muted-foreground underline">
            Sign in
          </Link>
        </div>
      )}
    </header>
  );
}
