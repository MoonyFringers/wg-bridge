---
status: "proposed"
date: 2025-12-07
decision-makers:
  - '@luca-c-xcv'
  - '@feed3r'
  - '@giubacc'
---

# WG-Bridge Python Project Structure

## Context and Problem Statement

WG-Bridge is a project that connects WireGuard endpoints reliably and 
efficiently, supporting custom properties for enhanced network management. The 
codebase is expected to grow (core orchestration, networking, config, CLI, UI) 
and attract community contributions.  

The primary questions are:

* How should the project be structured to support modularity, testing, 
packaging, and community contributions?
* Which layout and tooling should be adopted to keep development, packaging, 
and CI/CD straightforward?

## Decision Drivers

* Modular, maintainable, and testable code.
* Lower contribution barrier for the community (familiar layout, simple 
onboarding).
* Clear separation between core logic, networking, configuration, and interfaces
(CLI/UI).
* Compatibility with modern Python packaging practices (PEP 517/518, 
`pyproject.toml`, ...).
* Good support for automated testing, linting, and CI/CD pipelines.

## Considered Options

* **Option A – `src/` layout with modular packages** (chosen)
* **Option B – Flat layout (package at repo root)**
* **Option C – Monolithic package with internal modules only**

---

## Decision Outcome

**Chosen option: Option A – `src/` layout with modular packages**, because it 
aligns with widely recommended modern Python packaging practices, helps avoid 
import and packaging pitfalls, and keeps responsibilities clearly separated for
contributors.

### Proposed Layout

```sh
wg-bridge/
├── src/
│   ├── core/
│   │    ├── config/       
│   │    └── network/     
│   ├── utils/       
│   ├── ui/          
│   ├── cli/
│   └── installer/      
├── tests/               
├── scripts/             
├── .github/workflows/   
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── pyproject.toml       
├── requirements.txt     
└── .env.example
```

---

## Consequences

* **Good**: Clear separation of concerns (`core`, `network``, `cli`, `ui`) 
improves readability and maintainability.
* **Good**: `src/` layout avoids common import and packaging issues and scales 
better as the project grows.
* **Good**: Easier for new contributors familiar with modern Python templates to
navigate and extend.
* **Good**: Testing and CI setup are straightforward, as `tests/` mirrors `src/`
and tools integrate cleanly with the layout.
* **Neutral**: Slightly more ceremony than a flat layout (one extra directory 
level).
* **Bad**: More directories and indirection might feel heavy for very small 
scripts or experiments.

---

## Pros and Cons of the Options

### Option A – `src/` layout with modular packages (chosen)

* **Description**: Keep installable code under `src/wg_bridge/`, with 
subpackages `core`, `network`, `utils`, `ui`, and `cli`. Tests live in `tests/` 
and mirror the package structure.

* **Good**:
  * Aligns with modern guidance for larger Python projects and reduces 
  import-path confusion.
  * Encourages clean boundaries between core logic, networking and interfaces.

* **Bad**:
  * Slightly more verbose structure than a flat layout.
  * Requires contributors to understand the `src/` pattern if they only know 
  flat layouts.

---

### Option B – Flat layout (package at repo root)

* **Description**: Place `wg_bridge/` directly under the repository root, 
alongside `tests/`, without a `src/` directory.

* **Good**:
  * Simpler for very small projects and one-off scripts.
  * Reduces one directory level, which can be marginally more convenient for 
  quick hacking.

* **Bad**:
  * More prone to import confusion (tests or local files shadowing installed 
  packages).
  * Less aligned with current recommendations for growing, installable packages
  * Harder to evolve into a polished, distributable package if the project grows
  significantly.

---

### Option C – Monolithic package (no subpackages)

* **Description**: Single `wg_bridge` package with modules like `bridge.py`, 
`network.py`, `ui.py`, without separate subdirectories.

* **Good**:
  * Minimal structure; fewer directories and moving parts for a very small 
  codebase.
  * Lower initial cognitive load when only a handful of modules exist.

* **Bad**:
  * Becomes hard to navigate as the project grows (large, crowded package).
  * Weaker separation of concerns; core logic, networking, CLI, and UI can 
  easily tangle.
  * Refactoring into subpackages later is disruptive for imports and external
  users.
