/*
Repository-provider contracts for Git-aware EDA-Diff features.

Providers expose repository history and file content without coupling the UI to
GitHub, a local Git implementation, or another hosting service.
*/

export interface RepositoryCommit {
  id: string;
  parents: string[];
  message: string;
  authorName?: string;
  authoredAt?: string;
}

export interface RepositoryProvider {
  readonly label: string;
  listCommits(limit?: number): Promise<RepositoryCommit[]>;
  readFile(commitId: string, path: string): Promise<ArrayBuffer | undefined>;
}
