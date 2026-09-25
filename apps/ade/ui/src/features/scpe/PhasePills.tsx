import type { SliceDetail } from './api';

export interface PhasePillsProps {
  slices: SliceDetail[];
  selectedPhase: number | null;
  onSelectPhase: (phase: number | null) => void;
}

export function PhasePills({ slices, selectedPhase, onSelectPhase }: PhasePillsProps) {
  if (slices.length === 0) return null;

  return (
    <div className="astryx-phase-pills flex items-center gap-1.5 overflow-x-auto py-1 select-none">
      <button
        type="button"
        onClick={() => onSelectPhase(null)}
        className={`px-2.5 py-1 rounded-full text-xs font-medium transition-colors ${
          selectedPhase === null
            ? 'bg-[var(--accent)] text-white'
            : 'bg-[var(--bg-card)] text-[var(--text-secondary)] hover:text-[var(--text-primary)] border border-[var(--border-subtle)]'
        }`}
      >
        All Slices
      </button>

      {slices.map((slice) => {
        const isSelected = selectedPhase === slice.number;
        return (
          <button
            key={slice.number}
            type="button"
            onClick={() => onSelectPhase(slice.number)}
            className={`px-2.5 py-1 rounded-full text-xs font-medium transition-colors flex items-center gap-1 whitespace-nowrap ${
              isSelected
                ? 'bg-[var(--accent)] text-white'
                : 'bg-[var(--bg-card)] text-[var(--text-secondary)] hover:text-[var(--text-primary)] border border-[var(--border-subtle)]'
            }`}
          >
            <span>Slice {slice.number}</span>
            {slice.citations.length > 0 && (
              <span className="opacity-70 text-[10px] font-mono">
                ({slice.citations.slice(0, 2).join(', ')})
              </span>
            )}
          </button>
        );
      })}
    </div>
  );
}
