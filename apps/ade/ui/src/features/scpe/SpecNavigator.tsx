import { useState } from 'react';
import { ChevronRight } from 'lucide-react';
import { useWorkspaceStore } from '../../stores';
import { StateChip } from '../../components';

export function SpecNavigator() {
  const { outline, isOutlineLoading, outlineError, activeEpicRef, selectEpic, loadOutline } =
    useWorkspaceStore();

  const [collapsedFeatures, setCollapsedFeatures] = useState<Record<string, boolean>>({});

  const toggleFeature = (slug: string) => {
    setCollapsedFeatures((prev) => ({
      ...prev,
      [slug]: !prev[slug],
    }));
  };

  if (isOutlineLoading && !outline) {
    return (
      <div className="astryx-panel-body p-4 text-xs text-[var(--text-muted)] animate-pulse">
        Scanning living specifications...
      </div>
    );
  }

  if (outlineError) {
    return (
      <div className="astryx-panel-body p-4 flex flex-col gap-2">
        <div className="text-xs text-rose-400">{outlineError}</div>
        <button
          type="button"
          onClick={() => void loadOutline()}
          className="astryx-btn astryx-btn-secondary text-xs self-start"
        >
          Retry
        </button>
      </div>
    );
  }

  const features = outline?.features ?? [];

  return (
    <div className="astryx-nav-tree flex flex-col gap-1 p-2 overflow-y-auto">
      {features.map((feature) => {
        const isCollapsed = !!collapsedFeatures[feature.slug];
        return (
          <div key={feature.slug} className="astryx-feature-group flex flex-col gap-0.5">
            <button
              type="button"
              onClick={() => toggleFeature(feature.slug)}
              className="flex items-center gap-1.5 px-2 py-1.5 rounded text-xs font-semibold text-[var(--text-secondary)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-hover)] text-left transition-colors"
            >
              <ChevronRight
                size={14}
                strokeWidth={1.5}
                className={`transition-transform text-[var(--text-muted)] ${
                  isCollapsed ? '' : 'rotate-90'
                }`}
              />
              <span className="truncate flex-1">{feature.title}</span>
              <span className="text-[10px] font-mono text-[var(--text-muted)]">
                {feature.epics.length} epics
              </span>
            </button>

            {!isCollapsed && (
              <div className="flex flex-col gap-0.5 ml-3 pl-2 border-l border-[var(--border-subtle)]">
                {feature.epics.map((epic) => {
                  const isSelected =
                    activeEpicRef?.feature === feature.slug && activeEpicRef?.epic === epic.slug;

                  return (
                    <button
                      key={epic.slug}
                      type="button"
                      onClick={() => void selectEpic(feature.slug, epic.slug)}
                      className={`flex flex-col gap-1 p-2 rounded-md text-left transition-all ${
                        isSelected
                          ? 'bg-[var(--bg-active)] text-[var(--text-primary)] font-medium shadow-sm'
                          : 'hover:bg-[var(--bg-hover)] text-[var(--text-secondary)]'
                      }`}
                    >
                      <div className="flex items-center justify-between gap-1 w-full">
                        <span className="text-xs truncate flex-1 leading-snug">{epic.title}</span>
                      </div>
                      <div className="flex items-center justify-between gap-1 w-full pt-0.5">
                        <StateChip state={epic.state} />
                        <span className="font-mono text-[10px] text-[var(--text-muted)]">
                          {epic.tasks_done}/{epic.tasks_total}
                        </span>
                      </div>
                    </button>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}
    </div>
  );
}
