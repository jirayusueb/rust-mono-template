import { createFileRoute } from "@tanstack/react-router";
import { RequireAuth } from "@/components/require-auth";
import { TodoApp } from "@/features/todo/components/todo-app";
import { AppShell } from "@/components/layout/app-shell";

export const Route = createFileRoute("/todo")({
  head: () => ({ meta: [{ title: "Todos" }] }),
  component: TodoPage,
});

function TodoPage() {
  return (
    <RequireAuth>
      <AppShell>
        <TodoApp />
      </AppShell>
    </RequireAuth>
  );
}
