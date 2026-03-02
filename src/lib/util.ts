import type { Versions } from "./types";
import * as semver from "semver";

export function communityUrl(username: string) {
    return `https://github.com/${username}`;
}

export function formatModName(name: string): string {
	return name.replace(/_/g, ' ');
}

export function getNewestModVersionSync(versions: Versions): string {
    return Object.keys(versions).sort(semver.compare)[
      Object.keys(versions).length - 1
    ];
  }