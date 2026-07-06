"use client";

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useNavigate } from "@tanstack/react-router";
import { useEffect } from "react";
import { client } from "@/lib/client";
import { queryKeys } from "@/lib/query-keys";

export function useSession() {
  return useQuery({
    queryKey: queryKeys.session,
    queryFn: () => client.auth.session(),
    retry: false,
  });
}

export function useSignOut() {
  const qc = useQueryClient();
  return async () => {
    await client.auth.signOut();
    qc.invalidateQueries({ queryKey: queryKeys.session });
  };
}

export function useRequireAuth() {
  const session = useSession();
  const navigate = useNavigate();
  useEffect(() => {
    if (!session.isLoading && !session.data) navigate({ to: "/sign-in", replace: true });
  }, [session.isLoading, session.data, navigate]);
  return session;
}
