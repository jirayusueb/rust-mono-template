import { parseAsInteger, parseAsString, parseAsStringLiteral } from "nuqs";

export const todoSearchParams = {
  search: parseAsString.withDefault(""),
  status: parseAsStringLiteral(["all", "pending", "completed"]).withDefault("all"),
  page: parseAsInteger.withDefault(1),
  pageSize: parseAsInteger.withDefault(10),
};

export const themeParam = parseAsStringLiteral(["light", "dark"]).withDefault("light");
