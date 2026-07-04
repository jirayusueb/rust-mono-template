"use client";

import { useState, type FormEvent } from "react";
import { useRouter, Link } from "waku";
import { authClient, useInvalidateSession } from "../lib/auth-client";
import type { ApiError } from "../lib/api";
import { Button } from "@/components/ui/button";

export function RegisterForm() {
  const { replace } = useRouter();
  const invalidate = useInvalidateSession();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError(null);
    setLoading(true);
    try {
      await authClient.signUp.emailPassword({
        email,
        password,
        ...(name ? { name } : {}),
      });
      await invalidate();
      replace("/");
    } catch (err) {
      const apiErr = err as ApiError;
      setError(apiErr?.error?.message ?? "Registration failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex min-w-72 flex-col gap-4">
      <h1 className="text-2xl font-bold tracking-tight">Create account</h1>
      <form onSubmit={handleSubmit} className="flex flex-col gap-3">
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
          className="h-9 rounded-md border px-3 text-sm outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
        />
        <input
          type="password"
          placeholder="Password (min 8 chars)"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          required
          minLength={8}
          className="h-9 rounded-md border px-3 text-sm outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
        />
        <input
          type="text"
          placeholder="Name (optional)"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="h-9 rounded-md border px-3 text-sm outline-none focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50"
        />
        {error && <p className="text-sm text-destructive">{error}</p>}
        <Button type="submit" disabled={loading}>
          {loading ? "Creating…" : "Register"}
        </Button>
      </form>
      <p className="text-sm text-muted-foreground">
        Have an account?{" "}
        <Link to="/login" className="underline">
          Sign in
        </Link>
      </p>
    </div>
  );
}
