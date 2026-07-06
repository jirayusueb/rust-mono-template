import { useQueryClient } from "@tanstack/react-query";
import { client } from "./client";
import { queryKeys } from "./query-keys";

export const authClient = {
  signIn: {
    emailPassword: client.auth.signIn,
  },
  signUp: {
    emailPassword: client.auth.signUp,
  },
  signOut: client.auth.signOut,
};

export function useInvalidateSession() {
  const qc = useQueryClient();
  return () => qc.invalidateQueries({ queryKey: queryKeys.session });
}
