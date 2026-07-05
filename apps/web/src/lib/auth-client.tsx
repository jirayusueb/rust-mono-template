"use client";

import { useEffect, type ReactNode } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "waku";
import { client } from "./client";

export const authClient = {
  signIn: {
    emailPassword: client.auth.signIn,
  },
  signUp: {
    emailPassword: client.auth.signUp,
  },
  signOut: client.auth.signOut,
  useSession: () =>
    useQuery({
      queryKey: ["session"],
      queryFn: () => client.auth.session(),
      retry: false,
    }),
};

export function useInvalidateSession() {
  const qc = useQueryClient();
  return () => qc.invalidateQueries({ queryKey: ["session"] });
}

export function useRequireAuth() {
  const session = authClient.useSession();
  const { replace } = useRouter();
  useEffect(() => {
    if (!session.isLoading && !session.data) replace("/sign-in");
  }, [session.isLoading, session.data, replace]);
  return session;
}

export function RequireAuth({ children }: { children: ReactNode }) {
  const session = useRequireAuth();
  if (session.isLoading) return null;
  if (!session.data) return null;
  return <>{children}</>;
}
