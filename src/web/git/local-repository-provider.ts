/*
Local repository provider placeholder for browser-side Git repositories.

The provider boundary exists now so future isomorphic-git or OPFS integration can
be added without changing views or EDA comparison code.
*/

import type { RepositoryCommit, RepositoryProvider } from "./repository-provider";

export class LocalRepositoryProvider implements RepositoryProvider {
  readonly label = "Local repository";

  async listCommits(_limit?: number): Promise<RepositoryCommit[]> {
    throw new Error("Local Git repository access is not implemented yet.");
  }

  async readFile(_commitId: string, _path: string): Promise<ArrayBuffer | undefined> {
    throw new Error("Local Git repository access is not implemented yet.");
  }
}
