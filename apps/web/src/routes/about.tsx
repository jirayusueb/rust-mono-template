import { createFileRoute, Link } from "@tanstack/react-router";
import { AppShell } from "@/components/layout/app-shell";

export const Route = createFileRoute("/about")({
  head: () => ({ meta: [{ title: "About" }] }),
  component: AboutPage,
});

function AboutPage() {
  return (
    <AppShell>
      <div>
        <h1 className="text-4xl font-bold tracking-tight">About Waku</h1>
        <p>The minimal React framework</p>
        <Link to="/" className="mt-4 inline-block underline">
          Return home
        </Link>
      </div>
    </AppShell>
  );
}
