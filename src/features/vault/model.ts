import { translate } from '../../lib/locale';
export const categories = {
  award: translate('奖项'),
  competition: translate('比赛'),
  paper: translate('论文'),
  project: translate('项目'),
  certificate: translate('证书'),
  open_source: translate('开源'),
  course_project: translate('课程项目'),
  research: translate('研究'),
  presentation: translate('演示'),
  other: translate('其他'),
};
export interface Artifact {
  id: string;
  title: string;
  description: string;
  kind: string;
  artifact_date: string | null;
  role: string;
  organization: string;
  skills: string[];
  technologies: string[];
  repository_url: string;
  project_url: string;
  paper_url: string;
  journey_id: string | null;
  competition_id: string | null;
  visibility: string;
  revision: number;
  archived_at: string | null;
  deleted_at: string | null;
}
export interface Snapshot {
  artifacts: Artifact[];
  journeys: { id: string; title: string }[];
  competitions: { id: string; title: string }[];
  limit: number;
}
export interface Attachment {
  id: string;
  name: string;
  size_bytes: number;
  sha256: string;
  mime_type: string;
}
export function emptyArtifact(): Artifact {
  return {
    id: crypto.randomUUID(),
    title: '',
    description: '',
    kind: 'project',
    artifact_date: null,
    role: '',
    organization: '',
    skills: [],
    technologies: [],
    repository_url: '',
    project_url: '',
    paper_url: '',
    journey_id: null,
    competition_id: null,
    visibility: 'private',
    revision: 0,
    archived_at: null,
    deleted_at: null,
  };
}
export function portfolio(records: Artifact[]) {
  return records.filter((a) => a.visibility === 'public' && !a.deleted_at && !a.archived_at);
}
export function tags(text: string) {
  return [
    ...new Set(
      text
        .split(/[,，]/)
        .map((s) => s.trim())
        .filter(Boolean),
    ),
  ];
}
