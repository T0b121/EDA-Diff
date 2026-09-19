/*
Browser entry point for the EDA-Diff application.

Use this file only to bootstrap the client application. Parsing, Git access,
rendering, and EDA-specific behavior should live in reusable modules instead of
accumulating here as the project grows.
*/

const status = document.querySelector("#build-status");

if (status) {
  status.textContent = "EDA-Diff is ready for the next implementation step.";
}
