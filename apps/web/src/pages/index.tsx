import { Link } from "waku";
import { Counter } from "../components/counter";
import { HealthCheck } from "@/components/health-check";
import { apiGet, type Health } from "@/lib/api";
import { RequireAuth } from "@/lib/auth-client";

export default async function HomePage() {
  const data = await getData();

  return (
    <div>
      <title>{data.title}</title>
      <h1 className="text-4xl font-bold tracking-tight">{data.headline}</h1>
      <p>{data.body}</p>
      <p>
        API status (SSR): <strong>{data.apiStatus}</strong>
      </p>
      <RequireAuth>
        <HealthCheck />
        <Counter />
        <Link to="/about" className="mt-4 inline-block underline">
          About page
        </Link>
      </RequireAuth>
    </div>
  );
}

const getData = async () => {
  // ponytail: server-side fetch proves cross-server wiring on initial load
  let apiStatus = "unavailable";
  try {
    const health = await apiGet<Health>("/health");
    apiStatus = health.status ?? apiStatus;
  } catch {
    // API server not running — show default
  }

  return {
    title: "Waku",
    headline: "Waku",
    body: "Hello world!",
    apiStatus,
  };
};

export const getConfig = async () => {
  return {
    render: "dynamic",
  } as const;
};
