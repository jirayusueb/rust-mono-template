import { contract, type Contract, type Route } from "./contract";

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
 * Factory: type-only signature — `<Contract>` anchors the type, the imported
 * `contract` value drives runtime. No explicit type parameter needed at call
 * sites (it defaults to `Contract`).
 */
export function createClient<T extends Record<string, Record<string, Route>> = Contract>(
  endpoint: string,
): TypedClient<T> {
  const call = async <I, O>(r: Route<I, O>, input?: I): Promise<O> => {
    let path = r.path;
    let body: unknown = input;
    if (input && typeof input === "object") {
      body = { ...(input as object) };
      // extract path params first
      path = path.replace(/\{(\w+)\}/g, (_, k: string) => {
        const v = String((body as Record<string, unknown>)[k]);
        delete (body as Record<string, unknown>)[k];
        return v;
      });
      // ponytail: remaining keys → query string for GET/DELETE
      if (r.method === "GET" || r.method === "DELETE") {
        const qs = new URLSearchParams(
          Object.entries(body as Record<string, unknown>)
            .filter(([, v]) => v !== undefined && v !== "")
            .map(([k, v]) => [k, String(v)]),
        ).toString();
        if (qs) path += `?${qs}`;
        body = undefined;
      }
    }
    const init: RequestInit = {
      method: r.method,
      credentials: "include",
    };
    if (body) {
      init.body = JSON.stringify(body);
      init.headers = { "content-type": "application/json" };
    }
    const resp = await fetch(`${endpoint}${path}`, init);
    if (resp.status === 204) return undefined as O;
    const json = await resp.json().catch(() => ({
      error: { code: "Unknown", message: resp.statusText },
    }));
    if (!resp.ok) throw json as ApiError;
    return json as O;
  };

  // Build typed client from the runtime contract definition
  const client = {} as TypedClient<T>;
  for (const [ns, routes] of Object.entries(contract))
    for (const [name, r] of Object.entries(routes as Record<string, Route>)) {
      const nsObj = ((client as Record<string, Record<string, unknown>>)[ns] ??= {});
      nsObj[name] = (input?: unknown) => call(r as Route<unknown, unknown>, input as never);
    }
  return client;
}

export const client = createClient<Contract>("http://localhost:3001/api");
