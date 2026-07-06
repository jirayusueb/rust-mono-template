import { createFileRoute } from "@tanstack/react-router";
import { SignInForm } from "@/features/auth/components/sign-in-form";

export const Route = createFileRoute("/sign-in")({
  head: () => ({ meta: [{ title: "Sign in" }] }),
  component: SignInPage,
});

function SignInPage() {
  return <SignInForm />;
}
