import type { ReactNode } from "react";
import { Outlet, createRootRoute, HeadContent, Scripts } from "@tanstack/react-router";
import appCss from "../styles.css?url";
import { Providers } from "@/components/providers";
import { ClientProviders } from "@/components/client-providers";
import { Toaster } from "@/components/ui/sonner";

export const Route = createRootRoute({
  head: () => ({
    meta: [
      { charSet: "utf-8" },
      { name: "viewport", content: "width=device-width, initial-scale=1" },
      { name: "description", content: "A calm place for your tasks." },
    ],
    links: [
      { rel: "stylesheet", href: appCss },
      { rel: "preconnect", href: "https://fonts.googleapis.com" },
      { rel: "preconnect", href: "https://fonts.gstatic.com", crossOrigin: "" },
      {
        rel: "stylesheet",
        href: "https://fonts.googleapis.com/css2?family=Nunito:ital,wght@0,400;0,700;1,400;1,700&display=swap",
      },
    ],
  }),
  component: RootComponent,
  notFoundComponent: () => <p>Not Found</p>,
});

function RootComponent() {
  return (
    <RootDocument>
      <ClientProviders>
        <Providers>
          <Outlet />
        </Providers>
        <Toaster />
      </ClientProviders>
    </RootDocument>
  );
}

function RootDocument({ children }: Readonly<{ children: ReactNode }>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <head>
        {/* ponytail: theme flash — reads ?theme= or prefers-color-scheme before hydration */}
        <script
          dangerouslySetInnerHTML={{
            __html: `(function(){try{var p=new URLSearchParams(location.search);var t=p.get("theme");if(t==="dark"||(!t&&matchMedia("(prefers-color-scheme:dark)").matches)){document.documentElement.classList.add("dark")}}catch(e){}})()`,
          }}
        />
        <HeadContent />
      </head>
      <body className="min-h-svh bg-background font-['Nunito'] text-foreground antialiased">
        {children}
        <Scripts />
      </body>
    </html>
  );
}
