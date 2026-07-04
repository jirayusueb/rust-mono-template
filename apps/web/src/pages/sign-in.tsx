import { SignInForm } from "../components/sign-in-form";

export default function SignInPage() {
  return (
    <div>
      <title>Sign in</title>
      <SignInForm />
    </div>
  );
}

export const getConfig = async () => {
  return { render: "static" } as const;
};
