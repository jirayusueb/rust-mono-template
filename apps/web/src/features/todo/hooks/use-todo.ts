"use client";

import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { useQueryStates } from "nuqs";
import { useMemo } from "react";
import { client } from "@/lib/client";
import { queryKeys } from "@/lib/query-keys";
import { todoSearchParams } from "@/lib/search-params";
import type { TodoResponse } from "@/lib/dto";
import type { StatusFilter } from "../types";

export function useTodoList() {
  const [{ search, status, page, pageSize }, setParams] = useQueryStates(todoSearchParams);

  const { data: todos, isLoading } = useQuery({
    queryKey: [...queryKeys.todos.all, search],
    queryFn: () => client.todos.list(),
  });

  const filtered = useMemo(() => {
    let result = todos ?? [];
    // ponytail: search + status filtered client-side — API list() takes no params
    if (search) {
      const q = search.toLowerCase();
      result = result.filter((t) => t.title.toLowerCase().includes(q));
    }
    if (status !== "all") {
      result = result.filter((t) => t.status === status);
    }
    // ponytail: newest first — updated_at desc
    return [...result].sort(
      (a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime(),
    );
  }, [todos, status, search]);

  const totalPages = Math.max(1, Math.ceil(filtered.length / pageSize));
  const currentPage = Math.min(page, totalPages);
  const paginated = filtered.slice((currentPage - 1) * pageSize, currentPage * pageSize);

  const handleSearchChange = (v: string) => setParams({ search: v, page: 1 });
  const handleStatusChange = (v: StatusFilter) => setParams({ status: v, page: 1 });
  const handlePageChange = (p: number) => setParams({ page: p });
  const handlePageSizeChange = (n: number) => setParams({ page: 1, pageSize: n });

  return {
    isLoading,
    paginated,
    totalPages,
    currentPage,
    pageSize,
    search,
    status,
    handleSearchChange,
    handleStatusChange,
    handlePageChange,
    handlePageSizeChange,
  };
}

export function useTodoMutations() {
  const qc = useQueryClient();
  const invalidate = () => qc.invalidateQueries({ queryKey: queryKeys.todos.all });

  const create = useMutation({
    mutationFn: (input: { title: string }) => client.todos.create(input),
    onSuccess: invalidate,
  });

  const update = useMutation({
    mutationFn: (input: { id: string; title?: string; status?: "pending" | "completed" }) =>
      client.todos.update(input),
    onSuccess: invalidate,
  });

  const remove = useMutation({
    mutationFn: (input: { id: string }) => client.todos.remove(input),
    onSuccess: invalidate,
  });

  return { create, update, remove };
}

export function useTodoItem(todo: TodoResponse) {
  const qc = useQueryClient();
  const invalidate = () => qc.invalidateQueries({ queryKey: queryKeys.todos.all });

  const toggle = useMutation({
    mutationFn: () =>
      client.todos.update({
        id: todo.id,
        status: todo.status === "completed" ? "pending" : "completed",
      }),
    onSuccess: invalidate,
  });

  const rename = useMutation({
    mutationFn: (title: string) => client.todos.update({ id: todo.id, title }),
    onSuccess: invalidate,
  });

  const remove = useMutation({
    mutationFn: () => client.todos.remove({ id: todo.id }),
    onSuccess: invalidate,
  });

  return { toggle, rename, remove, invalidate };
}
