"use client";

import type { TodoResponse } from "@/lib/dto";
import { useTodoItem } from "../hooks/use-todo";
import { TodoItemView } from "./todo-item-view";

export function TodoItem({ todo }: { todo: TodoResponse }) {
  const { toggle, rename, remove } = useTodoItem(todo);
  return (
    <TodoItemView
      todo={todo}
      toggle={() => toggle.mutate()}
      rename={(title) => rename.mutate(title)}
      remove={() => remove.mutate()}
    />
  );
}
