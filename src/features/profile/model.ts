export interface Profile {
  display_name: string;
  headline: string;
  about: string;
  links: { label: string; url: string }[];
  public: boolean;
  revision: number;
}
export const emptyProfile: Profile = {
  display_name: '',
  headline: '',
  about: '',
  links: [],
  public: false,
  revision: 0,
};
export interface Projection {
  review_hash: string;
  schema_version: string;
  exported_at: string;
  profile: Profile | null;
  artifacts: { id: string; title: string; kind: string; public: true }[];
}
export interface Receipt {
  id: string;
  directory: string;
  sha256: string;
  artifacts: number;
}
