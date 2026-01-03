# Contributing Guide

Thank you for contributing to the Rust Data Engineering sandbox! This guide explains how to propose changes, submit assignments, and keep quality high.

## Ground Rules
- Be respectful and constructive in discussions and reviews.
- Prefer small, focused pull requests.
- Write tests for new behavior; keep tests passing.
- Run format and lint before pushing.
- Avoid committing secrets or credentials.

## Getting Started
1) Fork or branch from `main`.
2) Clone and install toolchain:
   ```bash
   git clone https://github.com/satoutahhaithem/Rust_for_data_engineering_template.git
   cd Rust_for_data_engineering_template
   rustc --version && cargo --version
   ```
3) Optional: open in VS Code / Codespaces for a preconfigured dev environment.

## Workflow
- Create a feature/bug branch: `git checkout -b feature/your-change`.
- Make changes in the appropriate week folder (e.g., `week_1/`).
- Run checks:
  ```bash
  make format
  make lint
  make test
  ```
- Commit with a clear message: `feat: add csv parser`.
- Push and open a Pull Request on GitHub.

## Pull Request Checklist
- [ ] Tests pass (`make test`).
- [ ] Code formatted (`make format`).
- [ ] No clippy warnings (`make lint`).
- [ ] Added/updated tests for new code.
- [ ] Updated docs if behavior changed.
- [ ] Linked related issue or milestone.

## Issue Labels (usage)
- `week-1`, `week-2`, `week-3`, ... : Week-specific items.
- `bug`: Something is broken.
- `question`: Ask for clarification/help.
- `documentation`: Docs-only changes.
- `help wanted` / `good first issue`: Suitable for newcomers.
- `priority-high|medium|low`: Urgency.

## Milestones
Attach issues/PRs to the relevant week milestone (e.g., "Week 1: Rust Basics") to track progress.

## Coding Standards
- Follow Rust idioms; prefer clear, safe code.
- Use `Result`/`Option` instead of panicking; avoid `unwrap()` in production code.
- Keep functions small and focused; add comments only where intent is non-obvious.
- Organize modules logically; avoid dead code.

## Testing Guidelines
- Add unit tests for new functions and edge cases.
- Use integration tests for public behavior in `tests/`.
- Prefer deterministic tests; avoid network calls unless mocked.

## Documentation
- Update `README.md` or week-specific `README.md` when behavior changes.
- Add doc comments (`///`) for public functions and structs.
- Include examples in docs when helpful.

## Asking for Help
- Open an issue with `question` label and relevant `week-X` label.
- Include context: what you tried, expected vs. actual behavior, error output.

## Security
- Do not commit secrets or API keys.
- Report security concerns privately (use GitHub security advisories if needed).

Happy coding! 🦀
