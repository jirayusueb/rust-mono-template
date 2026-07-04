import { RegisterForm } from "../components/register-form";

export default function RegisterPage() {
  return (
    <div>
      <title>Register</title>
      <RegisterForm />
    </div>
  );
}

export const getConfig = async () => {
  return { render: "static" } as const;
};
