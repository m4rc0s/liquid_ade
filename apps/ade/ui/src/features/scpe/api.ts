export interface EpicOutline {
  slug: string;
  title: string;
  state: string;
  tasks_done: number;
  tasks_total: number;
  path: string;
}

export interface FeatureOutline {
  slug: string;
  title: string;
  path: string;
  epics: EpicOutline[];
}

export interface ScpeOutline {
  features: FeatureOutline[];
}

export interface RuleDetail {
  id: string;
  title?: string;
  description: string;
}

export interface ExampleDetail {
  id: string;
  title: string;
  given: string;
  when: string;
  then: string;
  raw: string;
}

export interface SliceDetail {
  number: number;
  title: string;
  citations: string[];
  raw: string;
}

export interface TaskDetail {
  done: boolean;
  label: string;
  text: string;
  examples: string[];
  rules: string[];
}

export interface DomainModelDetail {
  terms: string[];
  entities: string[];
  events: string[];
  raw: string;
}

export interface EpicDetail {
  feature: string;
  epic: string;
  title: string;
  state: string;
  intent: string;
  domain_model: DomainModelDetail;
  rules: RuleDetail[];
  examples: ExampleDetail[];
  slices: SliceDetail[];
  open_questions: string[];
  tasks: TaskDetail[];
  raw_sections: Record<string, string>;
}

async function unwrap<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const errorBody = (await response.json().catch(() => null)) as { message?: string } | null;
    throw new Error(errorBody?.message ?? `HTTP error ${response.status}`);
  }
  return (await response.json()) as T;
}

export async function fetchScpeOutline(): Promise<ScpeOutline> {
  return unwrap<ScpeOutline>(await fetch('/api/scpe/outline'));
}

export async function fetchScpeEpic(feature: string, epic: string): Promise<EpicDetail> {
  return unwrap<EpicDetail>(
    await fetch(
      `/api/scpe/epic?feature=${encodeURIComponent(feature)}&epic=${encodeURIComponent(epic)}`
    )
  );
}
