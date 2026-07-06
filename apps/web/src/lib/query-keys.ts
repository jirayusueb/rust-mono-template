export const queryKeys = {
  session: ["session"] as const,
  todos: {
    all: ["todos"] as const,
    filters: (search: string, status: string) => [...queryKeys.todos.all, search, status] as const,
  },
} as const;
