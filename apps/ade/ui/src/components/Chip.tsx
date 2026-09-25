import type { HTMLAttributes, ReactNode } from 'react';

export type ChipVariant = 'draft' | 'ready' | 'wip' | 'blocked' | 'done' | 'stale' | 'default';

export interface ChipProps extends HTMLAttributes<HTMLSpanElement> {
  variant?: ChipVariant;
  children: ReactNode;
  className?: string;
}

export function Chip({ variant = 'default', children, className = '', ...props }: ChipProps) {
  const variantClass = variant !== 'default' ? variant : '';
  const classes = ['astryx-chip', variantClass, className].filter(Boolean).join(' ');
  return (
    <span className={classes} {...props}>
      {children}
    </span>
  );
}
