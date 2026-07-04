import { SignUpForm } from "../components/sign-up-form";

export default function SignUpPage() {
  return (
    <div>
      <title>Sign up</title>
      <SignUpForm />
    </div>
  );
}

export const getConfig = async () => {
  return { render: "static" } as const;
};
