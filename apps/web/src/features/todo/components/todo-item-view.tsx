"use client";

import { useState, useRef, useEffect } from "react";
import { Check, Pencil, Trash2, X } from "lucide-react";
import type { TodoResponse } from "@/lib/dto";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Input } from "@/components/ui/input";

export function TodoItemView({
  todo,
  toggle,
  rename,
  remove,
}: {
  todo: TodoResponse;
  toggle: () => void;
  rename: (title: string) => void;
  remove: () => void;
}) {
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState(todo.title);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (editing) {
      inputRef.current?.focus();
      inputRef.current?.select();
    }
  }, [editing]);

  const isDone = todo.status === "completed";
  const startEdit = () => {
    setDraft(todo.title);
    setEditing(true);
  };
  const commitEdit = () => {
    const next = draft.trim();
    if (next && next !== todo.title) rename(next);
    setEditing(false);
  };
  const cancelEdit = () => {
    setDraft(todo.title);
    setEditing(false);
  };

  return (
    <div className="group flex items-center gap-3 py-2.5">
      <Checkbox
        checked={isDone}
        onCheckedChange={toggle}
        aria-label={isDone ? "Mark as pending" : "Mark as completed"}
      />

      {editing ? (
        <form
          onSubmit={(e) => {
            e.preventDefault();
            commitEdit();
          }}
          className="flex flex-1 items-center gap-2"
        >
          <Input
            ref={inputRef}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Escape") cancelEdit();
            }}
            maxLength={200}
            className="h-8"
          />
          <Button type="submit" size="icon-sm" variant="ghost" aria-label="Save">
            <Check />
          </Button>
          <Button
            type="button"
            size="icon-sm"
            variant="ghost"
            onClick={cancelEdit}
            aria-label="Cancel"
          >
            <X />
          </Button>
        </form>
      ) : (
        <div className="flex flex-1 items-center gap-2">
          <span
            className={`flex-1 text-sm ${isDone ? "text-muted-foreground line-through" : ""}`}
            onDoubleClick={startEdit}
          >
            {todo.title}
          </span>
          <Badge variant="secondary" className="text-[11px] uppercase tracking-wide">
            {isDone ? "Done" : "Pending"}
          </Badge>
        </div>
      )}

      {!editing && (
        <div className="flex items-center gap-0.5 opacity-0 transition-opacity group-hover:opacity-100 focus-within:opacity-100">
          <Button size="icon-sm" variant="ghost" onClick={startEdit} aria-label="Edit todo">
            <Pencil />
          </Button>
          <Button
            size="icon-sm"
            variant="ghost"
            onClick={remove}
            className="text-muted-foreground hover:text-destructive"
            aria-label="Delete todo"
          >
            <Trash2 />
          </Button>
        </div>
      )}
    </div>
  );
}
