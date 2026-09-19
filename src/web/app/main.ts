/*
Browser entry point for the EDA-Diff single-page application.

This module only coordinates the initial UI. Domain parsing, repository access,
storage, and worker communication belong to their dedicated abstractions.
*/

const status = document.querySelector<HTMLElement>("#build-status");

if (status) {
  status.textContent = "EDA-Diff TypeScript application loaded.";
}
