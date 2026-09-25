import type { HTMLAttributes, ReactNode } from 'react';

export interface TaskItemProps extends Omit<HTMLAttributes<HTMLDivElement>, 'onToggle'> {
  id?: string;
  text: ReactNode;
  completed?: boolean;
  badges?: string[] | string;
  onToggle?: (completed: boolean) => void;
  className?: string;
}

export function TaskItem({
  id,
  text,
  completed = false,
  badges,
  onToggle,
  className = '',
  ...props
}: TaskItemProps) {
  const badgeList: string[] = Array.isArray(badges)
    ? badges
    : typeof badges === 'string'
      ? badges
          .split(',')
          .map((b) => b.trim())
          .filter(Boolean)
      : [];

  return (
    <div className={`astryx-task-item ${className}`.trim()} data-task-id={id} {...props}>
      <input
        type="checkbox"
        className="astryx-task-checkbox"
        checked={completed}
        onChange={(e) => onToggle?.(e.target.checked)}
        aria-label={typeof text === 'string' ? text : (id ?? 'Task item')}
      />
      <span className="astryx-task-text">{text}</span>
      {badgeList.length > 0 && (
        <div className="astryx-task-badges flex items-center gap-1">
          {badgeList.map((badge, idx) => (
            <span key={idx} className="astryx-task-badge">
              {badge}
            </span>
          ))}
        </div>
      )}
    </div>
  );
}
