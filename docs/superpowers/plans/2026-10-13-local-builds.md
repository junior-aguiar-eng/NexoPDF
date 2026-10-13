# NexoPDF local build infrastructure

**Goal:** Keep NexoPDF-specific automation and the fonts currently embedded from an external checkout inside this repository.

**Architecture:** Original GitHub workflows call the existing packaging scripts directly. Allowed, unmodified font files are vendored from their pinned original sources; the font build reads the local manifest without network access. Generic toolchains and GitHub Actions remain external, version-pinned tools.

**Constraints:** Preserve platform coverage, opt-in MCP, signing configuration, draft-only releases, original authorship and licences. Do not copy unlicensed automation. Do not include the Adobe-authored Noto CJK face. Do not run a release, install locally, or change secrets.

## Execution

- [x] Vendor the six permitted faces and their OFL notices; verify existing SHA-256 values and register each asset.
- [x] Embed local fonts by default and include their notices in every package, including MSI.
- [x] Replace the four externally delegated workflows with original local jobs; remove external font checkout from CI.
- [x] Normalize Linux desktop metadata to LF; validate workflows, scripts, attribution, fonts and affected Rust code.
- [x] Record actual checks and limitations. Publication remains pending the repository's green-test rule.

## Review focus

- A normal build must embed the local fonts without an environment variable.
- WASM must retain the existing small-face selection.
- All packaged distributions must carry the OFL notices.
- Changing an external project-specific workflow must no longer change NexoPDF's pipeline.
- Release overrides must be validated and must not enter shell commands as expressions.
- No release should be published automatically: generated releases stay drafts.

## Decisions

- The owner's request to store everything here overrides the previous shared-font location rule in AGENTS.md and CLAUDE.md.
- The former shared automation repository declares no licence. Its code will not be copied; new workflows use the packaging interfaces already present in NexoPDF.
- Existing Cargo dependencies, GitHub runners, packaging tools and attributed OCR model downloads are outside this migration. This change removes the two identified project-specific dependencies, not all software dependencies.
- GitHub currently has no configured release environment. The local workflow uses explicit release-branch conditions instead of creating one implicitly. No secrets or account settings were changed.

## Initial validation â€” 2026-10-13, before the IPC fix

- Six font programs downloaded from their pinned original sources: 25,204,828 bytes, all SHA-256 values identical to the previously pinned input. Five original OFL notices retained.
- Asset gate and generated attribution: 256 repository assets, six bundled dependency assets, 22 declared fetched assets; no external font build input remains.
- Font/xtask tests: 90 passed. Document-font save/reopen checks: two passed. Python version/draft safeguards: three passed.
- `cargo test --offline --locked --workspace --no-fail-fast`: 2,829 passed, three failed, 14 ignored. The three failures are the existing local Windows single-instance socket tests; no other test target failed. Tests were not edited or skipped to conceal them.
- Format, workflow syntax (actionlint 1.7.12), ShellCheck 0.11.0, PowerShell script and inline-workflow parsing, XML/Flatpak/local-font contracts and whitespace checks passed.
- Final whole-workspace Clippy with all targets and `-D warnings` passed (36.40 seconds).
- At this initial checkpoint, no local WASM target was installed. No installer build, installation, signing, release dispatch or publication was performed. The new workflows still require execution on their platform runners.
- At this initial checkpoint, no commit or push was made: CLAUDE.md requires green states before committing and the full local test gate was red for the three socket failures.

## IPC follow-up â€” 2026-10-13

- The socket-address-size hypothesis was falsified: the original dependency works outside AppData, while the independent .NET implementation reproduces Winsock 10022 inside AppData/Temp. The dependency experiment was removed from the active project; Cargo.toml and Cargo.lock are unchanged.
- Only Windows IPC storage moved to `%USERPROFILE%/.nexopdf-ipc`, preserving AF_UNIX, per-session/portable separation and safe fallback. The test fixtures use the same real production directory; no tests are skipped or mocked. Original failures and a new Unicode exchange pass: nine tests, zero failures.
- Real native process validation passed using isolated portable data: a second launch exited successfully and both synthetic PDFs appeared in the first window, including an accented/Japanese filename.
- Native workspace build passed. The local WASM target and the dependencies already declared in Cargo.lock were downloaded for validation; no package versions changed.
- One full-suite run during concurrent WASM compilation exceeded the existing 40 ms UI frame budget. That run is retained as evidence; the timing test and complete suite will be repeated with no compiler running, without changing the threshold.
- Final official `xtask wasm` check passed for all 31 selected libraries. The native workspace build also completed successfully. Dependency versions and Cargo.lock remain unchanged.
- Timing comparison on the same machine: the previous executable passed at 32.58 ms scrolling / 35.89 ms comments; the current executable passed at 30.39 ms / 33.20 ms. Earlier timing failures are retained. The complete suite and CI use one independent test at a time to avoid competing UI harnesses; the election regression still launches eight threads concurrently inside its test. No workloads, assertions, thresholds or fixtures are removed.

## Final validation — 2026-10-13

- Full locked offline workspace suite, with --test-threads=1: 2,833 passed, zero failed, 14 pre-existing ignored tests. The original performance limits and concurrent IPC election assertions passed.
- Final whole-workspace Clippy, all targets and warnings denied, passed. Native build, official WASM check (31 libraries), formatting, asset attribution and packaging/workflow checks passed as recorded above.
- Delivery is split into independent IPC, local-font and local-automation commits. Platform runner execution is checked after push; release dispatch, installer installation and publication were not performed locally.

## GitHub follow-up - 2026-10-13

- All four runs on aa6212e completed: CI, Packaging lint and Windows ARM64 passed. FreeBSD failed only in the real asset-manifest test because Git was absent inside the VM. The error was `running git ls-files: No such file or directory`.
- Commit ae909d3 installs Git in both FreeBSD VM preparation commands and checks its version before the full suite. Local validation rejects the missing executable dependency in either workflow; 47 xtask tests and three Python tests passed, as did actionlint, formatting and Clippy. Product code and test assertions are unchanged.
- Remote tests are rerun after push. The release workflow itself was not dispatched, and no release was published.

- The rerun on 0d6b377 passed CI and Windows ARM64, but FreeBSD still failed on `git ls-files failed`: the executable now exists, but the root VM reads a checkout whose host owner is preserved by rsync. The exact-path Git process configuration is applied to both VMs, with a checkout-read preflight before compilation. A synthetic different-owner Git checkout reproduces rejection and passes with the same scoped configuration. No wildcard or persistent user Git setting is used.

- On 7aec6c3, Packaging lint, CI and Windows ARM64 passed. FreeBSD passed the Git-read preflight, then compilation was killed with exit 137 before tests began. The VM default was 6144 MiB. Both FreeBSD jobs now request 8192 MiB and use one Cargo compiler job to reduce peak memory pressure; tests and performance limits are unchanged. Local workflow contracts, Python tests and actionlint passed. A fresh platform run is required.
