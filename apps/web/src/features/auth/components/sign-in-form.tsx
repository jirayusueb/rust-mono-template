"use client";

import { useSignInForm } from "../hooks/use-sign-in-form";
import { SignInView } from "../components/sign-in-view";

export function SignInForm() {
  return <SignInView {...useSignInForm()} />;
}
