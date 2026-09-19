/*
Hash router for the EDA-Diff single-page application.

GitHub Pages cannot rewrite arbitrary application routes to index.html, so this
router uses URL fragments and maps them to lightweight client-side views.
*/

export type AppRoute = "home" | "compare" | "repository";

const ROUTES: Record<string, AppRoute> = {
  "/": "home",
  "/compare": "compare",
  "/repository": "repository"
};

export function currentRoute(): AppRoute {
  const path = window.location.hash.slice(1) || "/";
  return ROUTES[path] ?? "home";
}

export function watchRoute(onRoute: (route: AppRoute) => void): () => void {
  const render = (): void => onRoute(currentRoute());

  window.addEventListener("hashchange", render);
  render();

  return () => window.removeEventListener("hashchange", render);
}
