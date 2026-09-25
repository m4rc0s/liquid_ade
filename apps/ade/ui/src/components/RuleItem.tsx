import type { HTMLAttributes, ReactNode } from 'react';

export type RuleType = 'rule' | 'scenario';

export interface RuleItemProps extends HTMLAttributes<HTMLDivElement> {
  id: string;
  type?: RuleType;
  title?: string;
  description: ReactNode;
  className?: string;
}

export function RuleItem({
  id,
  type,
  title,
  description,
  className = '',
  ...props
}: RuleItemProps) {
  const resolvedType = type ?? (id.toUpperCase().startsWith('R') ? 'rule' : 'scenario');
  const tagClass = resolvedType === 'rule' ? 'astryx-rule-tag' : 'astryx-scenario-tag';

  return (
    <div className={`astryx-inspector-item ${className}`.trim()} {...props}>
      <div className="astryx-inspector-item-head">
        <span className={tagClass}>{id}</span>
        {title && <span className="astryx-inspector-title">{title}</span>}
      </div>
      <div className="astryx-inspector-desc">{description}</div>
    </div>
  );
}
