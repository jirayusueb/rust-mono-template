"use client";

import { useSignUpForm } from "../hooks/use-sign-up-form";
import { SignUpView } from "../components/sign-up-view";

export function SignUpForm() {
  return <SignUpView {...useSignUpForm()} />;
}
