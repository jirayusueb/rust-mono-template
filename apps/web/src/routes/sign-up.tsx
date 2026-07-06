import { createFileRoute } from "@tanstack/react-router";
import { SignUpForm } from "@/features/auth/components/sign-up-form";

export const Route = createFileRoute("/sign-up")({
  head: () => ({ meta: [{ title: "Sign up" }] }),
  component: SignUpPage,
});

function SignUpPage() {
  return <SignUpForm />;
}
