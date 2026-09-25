import type { HTMLAttributes, ReactNode } from 'react';
import { Card as AstryxCoreCard } from '@astryxdesign/core';

export interface SpecCardProps extends Omit<HTMLAttributes<HTMLDivElement>, 'title'> {
  title?: ReactNode;
  children: ReactNode;
  actions?: ReactNode;
  className?: string;
}

export function SpecCard({ title, children, actions, className = '', ...props }: SpecCardProps) {
  return (
    <AstryxCoreCard className={`astryx-card ${className}`.trim()} {...props}>
      {title && (
        <div className="astryx-card-header flex items-center justify-between gap-2">
          <div className="astryx-card-title">{title}</div>
          {actions && <div className="astryx-card-actions">{actions}</div>}
        </div>
      )}
      <div className="astryx-card-content">{children}</div>
    </AstryxCoreCard>
  );
}
