import type {
  SignInRequest,
  SignUpRequest,
  UserResponse,
  SessionResponse,
  CreateTodoRequest,
  CreateTodoResponse,
  UpdateTodoRequest,
  TodoResponse,
} from "./contract";

export type Health = { status: string };

/** Route definition — phantom I/O type params carry contract types, runtime holds method + path. */
interface Route<_I = void, _O = void> {
  method: string;
  path: string;
}

const route = <I = void, O = void>(method: string, path: string): Route<I, O> => ({
  method,
  path,
});

export const appRouter = {
  auth: {
    signUp: route<SignUpRequest, UserResponse>("POST", "/auth/sign-up"),
    signIn: route<SignInRequest, UserResponse>("POST", "/auth/sign-in"),
    signOut: route("POST", "/auth/sign-out"),
    session: route<void, SessionResponse | null>("GET", "/auth/session"),
  },
  todos: {
    list: route<void, TodoResponse[]>("GET", "/todos"),
    create: route<CreateTodoRequest, CreateTodoResponse>("POST", "/todos"),
    get: route<{ id: string }, TodoResponse>("GET", "/todos/{id}"),
    update: route<{ id: string } & UpdateTodoRequest, void>("PATCH", "/todos/{id}"),
    remove: route<{ id: string }, void>("DELETE", "/todos/{id}"),
  },
  health: {
    check: route<void, Health>("GET", "/health"),
  },
} as const;

export type { Route };
