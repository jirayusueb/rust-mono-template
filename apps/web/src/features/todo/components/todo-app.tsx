"use client";

import { Card, CardContent } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Skeleton } from "@/components/ui/skeleton";
import { useTodoList } from "../hooks/use-todo";
import { TodoCreateForm } from "./todo-create-form";
import { TodoFilters } from "./todo-filters";
import { TodoItem } from "./todo-item";
import { TodoPagination } from "./todo-pagination";

export function TodoApp() {
  const {
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
  } = useTodoList();

  return (
    <Card className="mx-auto w-full max-w-2xl border-0 bg-transparent shadow-none">
      <CardContent className="flex flex-col gap-6 px-0 py-0">
        <header className="flex flex-col gap-1">
          <h1 className="text-3xl font-semibold tracking-tight">Todos</h1>
          <p className="text-sm text-muted-foreground">Capture, track, and complete your tasks.</p>
        </header>

        <TodoCreateForm />

        <Separator />
        <TodoFilters
          search={search}
          onSearchChange={handleSearchChange}
          status={status}
          onStatusChange={handleStatusChange}
        />

        <div className="flex min-h-48 flex-col">
          {isLoading ? (
            <div className="flex flex-col gap-3">
              {[0, 1, 2, 3].map((n) => (
                <div key={`skel-${n}`} className="flex items-center gap-3 py-2.5">
                  <Skeleton className="size-4 rounded-[4px]" />
                  <Skeleton className="h-4 flex-1" />
                  <Skeleton className="h-5 w-14 rounded-full" />
                </div>
              ))}
            </div>
          ) : paginated.length === 0 ? (
            <div className="flex flex-1 flex-col items-center justify-center gap-1 py-16 text-center">
              <p className="text-sm font-medium">No tasks yet</p>
              <p className="text-sm text-muted-foreground">
                Add your first task above to get started.
              </p>
            </div>
          ) : (
            <div className="flex flex-col divide-y">
              {paginated.map((todo) => (
                <TodoItem key={todo.id} todo={todo} />
              ))}
            </div>
          )}
        </div>

        <Separator />
        <TodoPagination
          page={currentPage}
          totalPages={totalPages}
          pageSize={pageSize}
          onPageChange={handlePageChange}
          onPageSizeChange={handlePageSizeChange}
        />
      </CardContent>
    </Card>
  );
}
