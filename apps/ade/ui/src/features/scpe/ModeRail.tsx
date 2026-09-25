import { Layers, FolderTree } from 'lucide-react';
import { useWorkspaceStore, type NavigatorMode } from '../../stores';

export function ModeRail() {
  const { activeMode, setActiveMode } = useWorkspaceStore();

  const modes: { id: NavigatorMode; label: string; icon: typeof Layers }[] = [
    { id: 'specs', label: 'Living Specs', icon: Layers },
    { id: 'files', label: 'Filesystem', icon: FolderTree },
  ];

  return (
    <nav className="astryx-mode-rail flex flex-col items-center py-3 bg-[var(--bg-surface)] border-r border-[var(--border-subtle)] w-12 min-w-12 select-none z-10">
      <div className="flex flex-col items-center gap-3 w-full">
        {modes.map((mode) => {
          const Icon = mode.icon;
          const isActive = activeMode === mode.id;
          return (
            <button
              key={mode.id}
              type="button"
              onClick={() => setActiveMode(mode.id)}
              className={`astryx-mode-btn relative flex items-center justify-center w-9 h-9 rounded-lg transition-all ${
                isActive
                  ? 'bg-[var(--accent)] text-white shadow-md shadow-indigo-500/20'
                  : 'text-[var(--text-muted)] hover:text-[var(--text-primary)] hover:bg-[var(--bg-hover)]'
              }`}
              title={mode.label}
              aria-label={mode.label}
              aria-pressed={isActive}
            >
              <Icon size={18} strokeWidth={1.5} />
              {isActive && (
                <span className="absolute left-0 top-2 bottom-2 w-0.5 bg-indigo-300 rounded-r" />
              )}
            </button>
          );
        })}
      </div>
    </nav>
  );
}
