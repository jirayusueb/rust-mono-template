import { appRouter, type Route } from "./router";

export type ApiError = { error: { code: string; message: string } };

// Type helpers — map Route<I, O> into callable (input) => Promise<output>
type Input<R> = R extends Route<infer I, any> ? I : void;
type Output<R> = R extends Route<any, infer O> ? O : void;
type Call<R extends Route> =
  Input<R> extends void ? () => Promise<Output<R>> : (input: Input<R>) => Promise<Output<R>>;

type TypedClient<T> = {
  [NS in keyof T]: T[NS] extends Record<string, Route>
    ? { [R in keyof T[NS]]: Call<T[NS][R]> }
    : never;
};

/**
 * Factory: takes the runtime router object, infers full client type from it.
 * Types are built from the runtime definition — no explicit type parameter needed.
 */
export function createClient<T extends Record<string, Record<string, Route>>>(
  router: T,
  endpoint: string,
  fetchOptions?: RequestInit,
): TypedClient<T> {
  const call = async <I, O>(r: Route<I, O>, input?: I): Promise<O> => {
    let path = r.path;
    let body: unknown = input;
    if (input && typeof input === "object") {
      body = { ...(input as object) };
      path = path.replace(/\{(\w+)\}/g, (_, k: string) => {
        const v = String((body as Record<string, unknown>)[k]);
        delete (body as Record<string, unknown>)[k];
        return v;
      });
      if (r.method === "GET" || r.method === "DELETE") body = undefined;
    }
    const init: RequestInit = {
      method: r.method,
      credentials: "include",
      ...fetchOptions,
    };
    if (body) {
      init.body = JSON.stringify(body);
      init.headers = { "content-type": "application/json", ...fetchOptions?.headers };
    }
    const resp = await fetch(`${endpoint}${path}`, init);
    if (resp.status === 204) return undefined as O;
    const json = await resp.json().catch(() => ({
      error: { code: "Unknown", message: resp.statusText },
    }));
    if (!resp.ok) throw json as ApiError;
    return json as O;
  };

  // Build typed client from the runtime router definition
  const client = {} as TypedClient<T>;
  for (const [ns, routes] of Object.entries(router))
    for (const [name, r] of Object.entries(routes as Record<string, Route>)) {
      const nsObj = ((client as Record<string, Record<string, unknown>>)[ns] ??= {});
      nsObj[name] = (input?: unknown) => call(r as Route<unknown, unknown>, input as never);
    }
  return client;
}

export const client = createClient(appRouter, "http://localhost:3001/api");
