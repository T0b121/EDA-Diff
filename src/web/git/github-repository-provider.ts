/*
GitHub repository provider for public repositories.

This module reads commit history, repository trees, and file contents through
the public GitHub API. Authentication and write operations are intentionally
outside this read-only provider.
*/

import { Octokit } from "@octokit/rest";
import type {
  RepositoryCommit,
  RepositoryProvider
} from "./repository-provider";

export interface GitHubRepositoryOptions {
  owner: string;
  repository: string;
  ref?: string;
}

export class GitHubRepositoryProvider implements RepositoryProvider {
  readonly label = "GitHub";
  private readonly client: Octokit;
  private readonly owner: string;
  private readonly repository: string;
  private readonly ref?: string;

  constructor(options: GitHubRepositoryOptions, token?: string) {
    this.client = new Octokit(token ? { auth: token } : undefined);
    this.owner = options.owner;
    this.repository = options.repository;
    this.ref = options.ref;
  }

  async listCommits(limit = 50): Promise<RepositoryCommit[]> {
    const response = await this.client.rest.repos.listCommits({
      owner: this.owner,
      repo: this.repository,
      sha: this.ref,
      per_page: Math.min(Math.max(limit, 1), 100)
    });

    return response.data.map((commit) => ({
      id: commit.sha,
      shortId: commit.sha.slice(0, 7),
      parents: commit.parents.map((parent) => parent.sha),
      message: commit.commit.message.split("\n", 1)[0],
      authorName: commit.commit.author?.name,
      authoredAt: commit.commit.author?.date ?? undefined
    }));
  }

  async readFile(
    commitId: string,
    path: string
  ): Promise<ArrayBuffer | undefined> {
    const response = await this.client.rest.repos.getContent({
      owner: this.owner,
      repo: this.repository,
      path,
      ref: commitId
    });

    if (!("content" in response.data) || response.data.type !== "file") {
      return undefined;
    }

    return decodeBase64(response.data.content);
  }

  async listFiles(commitId: string, path = ""): Promise<string[]> {
    const response = await this.client.rest.git.getTree({
      owner: this.owner,
      repo: this.repository,
      tree_sha: commitId,
      recursive: "true"
    });

    return response.data.tree
      .filter((entry) => entry.type === "blob")
      .map((entry) => entry.path)
      .filter((entry): entry is string =>
        entry !== undefined &&
        (path === "" || entry === path || entry.startsWith(path + "/"))
      );
  }
}

function decodeBase64(content: string): ArrayBuffer {
  const binary = atob(content.replace(/\s/g, ""));
  const bytes = new Uint8Array(binary.length);

  for (let index = 0; index < binary.length; index += 1) {
    bytes[index] = binary.charCodeAt(index);
  }

  return bytes.buffer;
}
