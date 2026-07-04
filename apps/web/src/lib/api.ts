const BASE_URL = "http://localhost:3001/api";

export async function apiGet<T>(path: string): Promise<T> {
  const resp = await fetch(`${BASE_URL}${path}`, { credentials: "include" });
  if (!resp.ok)
    throw (await resp
      .json()
      .catch(() => ({ error: { code: "Unknown", message: resp.statusText } }))) as ApiError;
  return resp.json() as Promise<T>;
}

export async function apiPost<T>(path: string, data?: unknown): Promise<T> {
  const init: RequestInit = {
    method: "POST",
    credentials: "include",
    headers: { "content-type": "application/json" },
  };
  if (data) init.body = JSON.stringify(data);
  const resp = await fetch(`${BASE_URL}${path}`, init);
  if (resp.status === 204) return undefined as T;
  const json = await resp
    .json()
    .catch(() => ({ error: { code: "Unknown", message: resp.statusText } }));
  if (!resp.ok) throw json as ApiError;
  return json as T;
}

export type User = {
  id: string;
  email: string;
  email_verified: boolean;
  name: string | null;
  image: string | null;
  created_at: string;
  updated_at: string;
};

export type Session = {
  expires_at: string;
  ip_address: string | null;
  user_agent: string | null;
  created_at: string;
};

export type SessionPayload = { user: User; session: Session };

export type ApiError = { error: { code: string; message: string } };

export type Health = { status: string };
