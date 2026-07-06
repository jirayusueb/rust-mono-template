"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { useTodoMutations } from "../hooks/use-todo";
import { TodoCreateFormView } from "./todo-create-form-view";

const schema = z.object({
  title: z.string().min(1, "Title is required").max(200, "Title is too long"),
});

export function TodoCreateForm() {
  const { create } = useTodoMutations();
  const {
    reset,
    watch,
    setValue,
    formState: { errors },
  } = useForm({
    resolver: zodResolver(schema),
    defaultValues: { title: "" },
  });

  const handleSubmit = () => {
    const title = watch("title").trim();
    if (title) {
      create.mutate({ title }, { onSuccess: () => reset() });
    }
  };

  return (
    <TodoCreateFormView
      value={watch("title")}
      onValueChange={(v) => setValue("title", v)}
      onSubmit={handleSubmit}
      isPending={create.isPending}
      error={errors.title?.message ?? undefined}
    />
  );
}
