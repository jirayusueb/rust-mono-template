"use client";

import { Link } from "@tanstack/react-router";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import { ThemeToggle } from "@/components/theme-toggle";
import type { SessionResponse } from "@/lib/dto";

export function HeaderView({
  session,
  isLoading,
  onSignOut,
}: {
  session: SessionResponse | undefined;
  isLoading: boolean;
  onSignOut: () => void;
}) {
  return (
    <header className="border-b">
      <div className="mx-auto flex h-14 max-w-3xl items-center justify-between px-6">
        <Link to="/" className="text-base font-semibold tracking-tight">
          Waku
        </Link>
        <nav className="flex items-center gap-1">
          {!isLoading && session && (
            <>
              <Button variant="ghost" size="sm" asChild>
                <Link to="/todo">Todos</Link>
              </Button>
              <Separator orientation="vertical" className="mx-1 h-4" />
              <span className="hidden text-sm text-muted-foreground sm:inline">
                {session.user.email}
              </span>
              <Button variant="ghost" size="sm" onClick={onSignOut}>
                Sign out
              </Button>
            </>
          )}
          {!isLoading && !session && (
            <>
              <Button variant="ghost" size="sm" asChild>
                <Link to="/sign-in">Sign in</Link>
              </Button>
              <Button variant="default" size="sm" asChild>
                <Link to="/sign-up">Get started</Link>
              </Button>
            </>
          )}
          <Separator orientation="vertical" className="mx-1 h-4" />
          <ThemeToggle />
        </nav>
      </div>
    </header>
  );
}
