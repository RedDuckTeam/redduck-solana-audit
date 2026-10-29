# Draft: Anthropic plugin directory submission

**Where:** https://claude.ai/directory/manage. A paid claude.ai plan is required.

**Before submitting:**

1. Add an icon:
   - square PNG, 512–2048 px per side, under 2 MB;
   - saved at `plugins/redduck-solana-audit/.claude-plugin/icon.png`, or set `"icon"` in `plugin.json`.

   The first icon submitted becomes the listing icon permanently.
2. Run `claude plugin validate --strict --profile directory plugins/redduck-solana-audit` from the repository root. It must pass with no problems.
3. Make sure the repository is public and tagged `v0.1.0`. README's Benchmark section is marked preliminary; keep that wording.

**Listing text:**

- **Name:** redduck-solana-audit
- **Summary (one line):** Architecture-level security review for Solana programs: 18 design patterns from the top protocols plus a classic-bug checklist.
- **Description:**
  > Audits Anchor, native and Pinocchio programs. Every instruction gets a root-of-trust pass. The skill then routes the program through the patterns that match its attack surface: oracle reads, operator-only actions, loops, signed messages, deferred actions and external dependencies. It finishes with an 11-item checklist of classic Solana bugs.
  >
  > A verifier agent tries to refute every High or Critical finding. The report lists findings with severity, location and mechanism, a separate section for design problems, and a coverage table.
  >
  > The skill contains no hooks, MCP servers or network calls. Preliminary benchmark and limits are in the README.
- **Category:** Security / Development
- **Source:** https://github.com/RedDuckTeam/redduck-solana-audit
