import type { HTMLAttributes } from 'react';
import { Chip, type ChipVariant } from './Chip';

export type SCPEState = 'Draft' | 'Ready' | 'WIP' | 'Blocked' | 'Done' | 'Stale';

export interface StateChipProps extends HTMLAttributes<HTMLSpanElement> {
  state: SCPEState | string;
  className?: string;
}

const STATE_VARIANT_MAP: Record<string, ChipVariant> = {
  draft: 'draft',
  ready: 'ready',
  wip: 'wip',
  blocked: 'blocked',
  done: 'done',
  stale: 'stale',
};

export function StateChip({ state, className = '', ...props }: StateChipProps) {
  const normalizedKey = state.toLowerCase();
  const variant = STATE_VARIANT_MAP[normalizedKey] ?? 'default';

  return (
    <Chip variant={variant} className={`astryx-state-chip ${className}`.trim()} {...props}>
      {state}
    </Chip>
  );
}
