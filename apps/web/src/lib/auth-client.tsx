"use client";

import { useEffect, type ReactNode } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useRouter } from "waku";
import { apiGet, apiPost, type SessionPayload } from "./api";

export const authClient = {
  signIn: {
    emailPassword: (body: { email: string; password: string }) =>
      apiPost<SessionPayload>("/auth/sign-in", body),
  },
  signUp: {
    emailPassword: (body: { email: string; password: string; name?: string }) =>
      apiPost<SessionPayload>("/auth/sign-up", body),
  },
  signOut: () => apiPost<void>("/auth/sign-out"),
  useSession: () =>
    useQuery({
      queryKey: ["session"],
      queryFn: () => apiGet<SessionPayload>("/auth/session"),
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
