import { useWorkspaceStore } from '../../stores';
import { StateChip, RuleItem } from '../../components';

export function SpecInspector() {
  const { activeEpicDetail, isEpicLoading } = useWorkspaceStore();

  if (isEpicLoading) {
    return (
      <div className="astryx-sidebar-right p-4 text-xs text-[var(--text-muted)] animate-pulse">
        Loading spec inspector...
      </div>
    );
  }

  if (!activeEpicDetail) {
    return (
      <div className="astryx-sidebar-right p-4 text-xs text-[var(--text-muted)]">
        No active epic selected. Select an epic in the navigator to inspect.
      </div>
    );
  }

  const epic = activeEpicDetail;

  return (
    <div className="astryx-sidebar-right flex flex-col gap-5 p-4 overflow-y-auto">
      {/* Overview */}
      <div className="flex flex-col gap-2 pb-3 border-b border-[var(--border-subtle)]">
        <span className="text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
          Epic Metadata
        </span>
        <div className="flex items-center justify-between">
          <span className="text-xs font-semibold text-[var(--text-primary)]">{epic.epic}</span>
          <StateChip state={epic.state} />
        </div>
        <div className="text-[11px] text-[var(--text-secondary)]">Feature: {epic.feature}</div>
      </div>

      {/* Stats Summary */}
      <div className="grid grid-cols-3 gap-2">
        <div className="flex flex-col items-center p-2 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)]">
          <span className="text-sm font-bold text-[var(--status-ready)]">{epic.rules.length}</span>
          <span className="text-[10px] text-[var(--text-muted)]">Rules</span>
        </div>
        <div className="flex flex-col items-center p-2 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)]">
          <span className="text-sm font-bold text-[var(--status-done)]">
            {epic.examples.length}
          </span>
          <span className="text-[10px] text-[var(--text-muted)]">Scenarios</span>
        </div>
        <div className="flex flex-col items-center p-2 rounded bg-[var(--bg-card)] border border-[var(--border-subtle)]">
          <span className="text-sm font-bold text-[var(--accent)]">{epic.slices.length}</span>
          <span className="text-[10px] text-[var(--text-muted)]">Slices</span>
        </div>
      </div>

      {/* Rules Section */}
      <div className="flex flex-col gap-2">
        <span className="text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
          Business Rules (R#)
        </span>
        <div className="flex flex-col gap-2">
          {epic.rules.map((rule) => (
            <RuleItem
              key={rule.id}
              id={rule.id}
              type="rule"
              description={<span className="text-xs">{rule.description}</span>}
            />
          ))}
        </div>
      </div>

      {/* Scenarios Section */}
      <div className="flex flex-col gap-2">
        <span className="text-[10px] font-bold uppercase tracking-wider text-[var(--text-muted)]">
          Acceptance Scenarios (S#)
        </span>
        <div className="flex flex-col gap-2">
          {epic.examples.map((ex) => (
            <RuleItem
              key={ex.id}
              id={ex.id}
              type="scenario"
              title={ex.title}
              description={
                <span className="text-xs line-clamp-3">
                  {ex.given ? `Given ${ex.given}... Then ${ex.then}` : ex.raw}
                </span>
              }
            />
          ))}
        </div>
      </div>
    </div>
  );
}
