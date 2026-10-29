// Run settings shared by every benchmark case. Changing any of these between
// parts of a full run makes the parts incomparable (plan, section "Metrics").

export const MAX_TURNS = 60;
export const TIMEOUT_SECONDS = 1800;
export const ALLOWED_TOOLS = ['Read', 'Glob', 'Grep', 'Skill', 'Agent', 'Write'];
export const REPORT_FILE = 'solana-audit-report.md';
export const GROUPS = ['g1', 'g2', 'g3'];
