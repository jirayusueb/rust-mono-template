"use client";

import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

export function TodoCreateFormView({
  value,
  onValueChange,
  onSubmit,
  isPending,
  error,
}: {
  value: string;
  onValueChange: (v: string) => void;
  onSubmit: () => void;
  isPending: boolean;
  error?: string | undefined;
}) {
  return (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        onSubmit();
      }}
      className="flex w-full items-start gap-2"
      noValidate
    >
      <div className="flex-1">
        <Input
          value={value}
          onChange={(e) => onValueChange(e.target.value)}
          placeholder="Add a task…"
          aria-label="New todo title"
          className="h-9"
        />
        {error && <p className="mt-1 text-xs text-destructive">{error}</p>}
      </div>
      <Button type="submit" disabled={isPending} className="h-9">
        {isPending ? "Adding…" : "Add"}
      </Button>
    </form>
  );
}
