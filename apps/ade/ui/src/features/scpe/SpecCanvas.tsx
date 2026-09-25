import { Sparkles } from 'lucide-react';
import { useWorkspaceStore } from '../../stores';
import { SpecCard, StateChip, RuleItem, TaskItem } from '../../components';
import { PhasePills } from './PhasePills';

export function SpecCanvas() {
  const {
    activeEpicRef,
    activeEpicDetail,
    isEpicLoading,
    epicError,
    selectedPhase,
    setSelectedPhase,
    selectEpic,
    dispatchTaskToCopilot,
  } = useWorkspaceStore();

  if (isEpicLoading) {
    return (
      <div className="astryx-canvas flex-1 p-8 flex flex-col gap-4 animate-pulse">
        <div className="h-6 w-1/3 bg-[var(--bg-card)] rounded" />
        <div className="h-28 bg-[var(--bg-card)] rounded-lg" />
        <div className="h-40 bg-[var(--bg-card)] rounded-lg" />
      </div>
    );
  }

  if (epicError) {
    return (
      <div className="astryx-canvas flex-1 p-8 flex flex-col gap-4">
        <div className="astryx-error-box">{epicError}</div>
        {activeEpicRef && (
          <button
            type="button"
            onClick={() => void selectEpic(activeEpicRef.feature, activeEpicRef.epic)}
            className="astryx-btn astryx-btn-secondary self-start"
          >
            Retry
          </button>
        )}
      </div>
    );
  }

  if (!activeEpicDetail) {
    return (
      <div className="astryx-canvas flex-1 p-8 flex items-center justify-center text-[var(--text-muted)] text-sm">
        Select an epic from the Spec Navigator to view living specifications.
      </div>
    );
  }

  const epic = activeEpicDetail;

  // Filter rules, examples, and tasks if a slice is selected
  const activeSlice = selectedPhase ? epic.slices.find((s) => s.number === selectedPhase) : null;

  const relevantCitations = new Set(activeSlice?.citations ?? []);

  const displayedRules =
    selectedPhase && relevantCitations.size > 0
      ? epic.rules.filter((r) => relevantCitations.has(r.id))
      : epic.rules;

  const displayedExamples =
    selectedPhase && relevantCitations.size > 0
      ? epic.examples.filter((e) => relevantCitations.has(e.id))
      : epic.examples;

  const displayedTasks =
    selectedPhase && relevantCitations.size > 0
      ? epic.tasks.filter(
          (t) =>
            t.rules.some((r) => relevantCitations.has(r)) ||
            t.examples.some((e) => relevantCitations.has(e))
        )
      : epic.tasks;

  return (
    <div className="astryx-canvas flex-1 overflow-y-auto p-7 flex flex-col gap-6 max-w-5xl">
      {/* Header & Meta */}
      <header className="astryx-doc-header flex flex-col gap-2 pb-4 border-b border-[var(--border-subtle)]">
        <div className="flex items-center gap-2 text-xs text-[var(--text-muted)]">
          <span>{epic.feature}</span>
          <span>/</span>
          <span className="text-[var(--text-secondary)]">{epic.epic}</span>
        </div>
        <div className="flex items-center justify-between gap-4">
          <h1 className="text-2xl font-bold tracking-tight text-[var(--text-primary)]">
            {epic.title}
          </h1>
          <StateChip state={epic.state} />
        </div>
      </header>

      {/* Phase Pills */}
      <PhasePills
        slices={epic.slices}
        selectedPhase={selectedPhase}
        onSelectPhase={setSelectedPhase}
      />

      {/* Intent Card */}
      {epic.intent && (
        <SpecCard title="Intent">
          <p className="text-sm leading-relaxed text-[var(--text-secondary)] whitespace-pre-line">
            {epic.intent}
          </p>
        </SpecCard>
      )}

      {/* Domain Model Card */}
      {(epic.domain_model.terms.length > 0 ||
        epic.domain_model.entities.length > 0 ||
        epic.domain_model.events.length > 0) && (
        <SpecCard title="Domain Model">
          <div className="flex flex-col gap-3 text-xs">
            {epic.domain_model.terms.length > 0 && (
              <div className="flex items-start gap-2 flex-wrap">
                <span className="font-semibold text-[var(--text-muted)] min-w-16">Terms:</span>
                <div className="flex flex-wrap gap-1.5">
                  {epic.domain_model.terms.map((t, i) => (
                    <span
                      key={i}
                      className="px-2 py-0.5 bg-[var(--bg-card)] border border-[var(--border-subtle)] rounded text-[var(--text-secondary)] font-medium"
                    >
                      {t}
                    </span>
                  ))}
                </div>
              </div>
            )}
            {epic.domain_model.entities.length > 0 && (
              <div className="flex items-start gap-2 flex-wrap">
                <span className="font-semibold text-[var(--text-muted)] min-w-16">Entities:</span>
                <div className="flex flex-wrap gap-1.5">
                  {epic.domain_model.entities.map((e, i) => (
                    <span
                      key={i}
                      className="px-2 py-0.5 bg-blue-950/40 text-blue-300 border border-blue-900/40 rounded font-medium font-mono text-[11px]"
                    >
                      {e}
                    </span>
                  ))}
                </div>
              </div>
            )}
            {epic.domain_model.events.length > 0 && (
              <div className="flex items-start gap-2 flex-wrap">
                <span className="font-semibold text-[var(--text-muted)] min-w-16">Events:</span>
                <div className="flex flex-wrap gap-1.5">
                  {epic.domain_model.events.map((ev, i) => (
                    <span
                      key={i}
                      className="px-2 py-0.5 bg-amber-950/40 text-amber-300 border border-amber-900/40 rounded font-medium font-mono text-[11px]"
                    >
                      {ev}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </div>
        </SpecCard>
      )}

      {/* Tasks Card with Action Pill dispatch */}
      {displayedTasks.length > 0 && (
        <SpecCard
          title={
            <div className="flex items-center justify-between w-full">
              <span>Execution Tasks</span>
              <span className="text-xs font-mono font-normal text-[var(--text-muted)]">
                {displayedTasks.filter((t) => t.done).length} / {displayedTasks.length} done
              </span>
            </div>
          }
        >
          <div className="flex flex-col gap-2">
            {displayedTasks.map((task, idx) => {
              const allBadges = [...task.rules, ...task.examples];
              return (
                <div
                  key={idx}
                  className="flex items-center justify-between gap-3 p-2.5 rounded-md bg-[var(--bg-card)] border border-[var(--border-subtle)]"
                >
                  <div className="flex-1 min-w-0">
                    <TaskItem
                      id={task.label}
                      text={
                        <span>
                          <strong className="text-[var(--text-primary)] mr-1.5">
                            {task.label}:
                          </strong>
                          {task.text}
                        </span>
                      }
                      completed={task.done}
                      badges={allBadges}
                      className="border-none bg-transparent p-0"
                    />
                  </div>

                  {/* Action Pill: Dispatch task to Conversational Co-Pilot (R6) */}
                  <button
                    type="button"
                    onClick={() => dispatchTaskToCopilot(task)}
                    className="astryx-action-pill flex items-center gap-1 px-2.5 py-1 rounded bg-indigo-600/15 hover:bg-indigo-600/25 text-indigo-400 hover:text-indigo-300 border border-indigo-500/30 text-xs font-medium transition-all whitespace-nowrap shadow-sm"
                    title={`Send ${task.label} to Co-Pilot`}
                  >
                    <Sparkles size={13} strokeWidth={1.5} />
                    <span>Ask Co-Pilot</span>
                  </button>
                </div>
              );
            })}
          </div>
        </SpecCard>
      )}

      {/* Business Rules Card */}
      {displayedRules.length > 0 && (
        <SpecCard title="Business Rules">
          <div className="flex flex-col gap-2">
            {displayedRules.map((rule) => (
              <RuleItem
                key={rule.id}
                id={rule.id}
                type="rule"
                title={rule.title}
                description={rule.description}
              />
            ))}
          </div>
        </SpecCard>
      )}

      {/* Acceptance Scenarios Card */}
      {displayedExamples.length > 0 && (
        <SpecCard title="Acceptance Scenarios">
          <div className="flex flex-col gap-2">
            {displayedExamples.map((ex) => (
              <RuleItem
                key={ex.id}
                id={ex.id}
                type="scenario"
                title={ex.title}
                description={
                  ex.given ? (
                    <div className="flex flex-col gap-1 mt-1 text-xs">
                      <div>
                        <strong className="text-zinc-400">Given</strong> {ex.given}
                      </div>
                      <div>
                        <strong className="text-zinc-400">When</strong> {ex.when}
                      </div>
                      <div>
                        <strong className="text-zinc-400">Then</strong> {ex.then}
                      </div>
                    </div>
                  ) : (
                    ex.raw
                  )
                }
              />
            ))}
          </div>
        </SpecCard>
      )}

      {/* Vertical Slices Card */}
      {epic.slices.length > 0 && (
        <SpecCard title="Vertical Slices">
          <div className="flex flex-col gap-2">
            {epic.slices.map((slice) => (
              <div
                key={slice.number}
                className="p-3 bg-[var(--bg-card)] border border-[var(--border-subtle)] rounded-md flex flex-col gap-1.5"
              >
                <div className="flex items-center justify-between text-xs font-semibold text-[var(--text-primary)]">
                  <span>
                    Slice {slice.number}: {slice.title}
                  </span>
                  {slice.citations.length > 0 && (
                    <div className="flex items-center gap-1 font-mono text-[10px]">
                      {slice.citations.map((c, i) => (
                        <span
                          key={i}
                          className="px-1.5 py-0.5 rounded bg-[var(--accent-glow)] text-[var(--accent)]"
                        >
                          {c}
                        </span>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        </SpecCard>
      )}

      {/* Open Questions */}
      {epic.open_questions.length > 0 && (
        <SpecCard title="Open Questions">
          <ul className="list-disc pl-5 text-xs text-[var(--text-secondary)] space-y-1">
            {epic.open_questions.map((q, i) => (
              <li key={i}>{q}</li>
            ))}
          </ul>
        </SpecCard>
      )}
    </div>
  );
}
