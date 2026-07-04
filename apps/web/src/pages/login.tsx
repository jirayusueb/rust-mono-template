import { LoginForm } from "../components/login-form";

export default function LoginPage() {
  return (
    <div>
      <title>Login</title>
      <LoginForm />
    </div>
  );
}

export const getConfig = async () => {
  return { render: "static" } as const;
};
