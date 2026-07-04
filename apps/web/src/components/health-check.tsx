"use client";

import { useQuery } from "@tanstack/react-query";
import { apiGet, type Health } from "@/lib/api";
import { Button } from "@/components/ui/button";
import { RefreshCw } from "lucide-react";

export function HealthCheck() {
  const { data, isLoading, isError, refetch } = useQuery({
    queryKey: ["health"],
    queryFn: () => apiGet<Health>("/health"),
    refetchInterval: 10_000,
  });

  return (
    <section className="mt-4 space-y-2">
      <div className="text-sm">
        API status (live):{" "}
        {isLoading ? (
          <span className="text-muted-foreground">checking…</span>
        ) : isError ? (
          <span className="text-destructive">unavailable</span>
        ) : (
          <strong>{data?.status}</strong>
        )}
      </div>
      <Button variant="outline" size="sm" onClick={() => refetch()} disabled={isLoading}>
        <RefreshCw className={isLoading ? "animate-spin" : ""} />
        Refresh
      </Button>
    </section>
  );
}
