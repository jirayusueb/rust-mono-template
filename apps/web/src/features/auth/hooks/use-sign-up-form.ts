"use client";

import { useNavigate } from "@tanstack/react-router";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { signUpSchema, type SignUpValues } from "../types/schemas";
import { authClient, useInvalidateSession } from "@/lib/auth-client";
import type { ApiError } from "@/lib/client";

export function useSignUpForm() {
  const navigate = useNavigate();
  const invalidate = useInvalidateSession();
  const [serverError, setServerError] = useState<string | null>(null);

  const form = useForm<SignUpValues>({ resolver: zodResolver(signUpSchema) });

  const onSubmit = async (values: SignUpValues) => {
    setServerError(null);
    try {
      await authClient.signUp.emailPassword({
        email: values.email,
        password: values.password,
        ...(values.name ? { name: values.name } : {}),
      });
      await invalidate();
      navigate({ to: "/", replace: true });
    } catch (err) {
      const apiErr = err as ApiError;
      setServerError(apiErr?.error?.message ?? "Sign up failed");
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
