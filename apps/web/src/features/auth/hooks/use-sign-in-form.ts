"use client";

import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { signInSchema, type SignInValues } from "../types/schemas";
import { authClient, useInvalidateSession } from "@/lib/auth-client";
import type { ApiError } from "@/lib/client";

export function useSignInForm() {
  const navigate = useNavigate();
  const invalidate = useInvalidateSession();
  const [serverError, setServerError] = useState<string | null>(null);

  const form = useForm<SignInValues>({ resolver: zodResolver(signInSchema) });

  const onSubmit = async (values: SignInValues) => {
    setServerError(null);
    try {
      await authClient.signIn.emailPassword(values);
      await invalidate();
      navigate({ to: "/", replace: true });
    } catch (err) {
      const apiErr = err as ApiError;
      setServerError(apiErr?.error?.message ?? "Sign in failed");
    }
  };

  return {
    form,
    onSubmit,
    errors: form.formState.errors,
    serverError,
    isSubmitting: form.formState.isSubmitting,
  };
}
