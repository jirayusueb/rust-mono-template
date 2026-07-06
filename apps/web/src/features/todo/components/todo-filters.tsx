"use client";

import type { StatusFilter } from "../types";
import { TodoFiltersView } from "./todo-filters-view";

export function TodoFilters({
  search,
  onSearchChange,
  status,
  onStatusChange,
}: {
  search: string;
  onSearchChange: (v: string) => void;
  status: StatusFilter;
  onStatusChange: (v: StatusFilter) => void;
}) {
  return (
    <TodoFiltersView
      search={search}
      onSearchChange={onSearchChange}
      status={status}
      onStatusChange={onStatusChange}
    />
  );
}
