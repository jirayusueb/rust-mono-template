import { createFileRoute } from "@tanstack/react-router";
import { Link } from "@tanstack/react-router";
import { CheckCircle2, ListChecks, Search } from "lucide-react";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { AppShell } from "@/components/layout/app-shell";

export const Route = createFileRoute("/")({
  head: () => ({ meta: [{ title: "Waku · Todos" }] }),
  component: HomePage,
});

function HomePage() {
  return (
    <AppShell>
      <div className="mx-auto flex w-full max-w-2xl flex-col gap-12">
        <section className="flex flex-col gap-5 pt-8">
          <h1 className="text-balance text-4xl font-semibold tracking-tight sm:text-5xl">
            A calm place for your tasks.
          </h1>
          <p className="max-w-prose text-pretty text-base text-muted-foreground">
            Capture what matters, filter by what's pending, and find anything with search. No
            clutter, no noise — just your list, done right.
          </p>
          <div className="flex flex-wrap items-center gap-3 pt-1">
            <Button asChild>
              <Link to="/sign-up">Get started free</Link>
            </Button>
            <Button variant="outline" asChild>
              <Link to="/sign-in">Sign in</Link>
            </Button>
          </div>
        </section>

        <section className="grid gap-4 sm:grid-cols-3">
          <Card className="border-0 bg-secondary/50 shadow-none">
            <CardContent className="flex flex-col gap-3 p-5">
              <Badge variant="outline" className="self-start rounded-lg p-0">
                <ListChecks className="size-5" />
              </Badge>
              <h2 className="text-sm font-semibold">Track everything</h2>
              <p className="text-sm text-muted-foreground">
                Create, complete, and edit tasks. Your list stays in sync.
              </p>
            </CardContent>
          </Card>
          <Card className="border-0 bg-secondary/50 shadow-none">
            <CardContent className="flex flex-col gap-3 p-5">
              <Badge variant="outline" className="self-start rounded-lg p-0">
                <Search className="size-5" />
              </Badge>
              <h2 className="text-sm font-semibold">Find instantly</h2>
              <p className="text-sm text-muted-foreground">
                Search by title and filter by status to focus on what's next.
              </p>
            </CardContent>
          </Card>
          <Card className="border-0 bg-secondary/50 shadow-none">
            <CardContent className="flex flex-col gap-3 p-5">
              <Badge variant="outline" className="self-start rounded-lg p-0">
                <CheckCircle2 className="size-5" />
              </Badge>
              <h2 className="text-sm font-semibold">Stay focused</h2>
              <p className="text-sm text-muted-foreground">
                Pagination keeps long lists manageable and scannable.
              </p>
            </CardContent>
          </Card>
        </section>
      </div>
    </AppShell>
  );
}
