<script lang="ts">
  import { getMod } from "$lib/api/commands";
  import type { ModEntry } from "$lib/types";
  import {
    communityUrl,
    formatModName,
    getNewestModVersionSync,
  } from "$lib/util";
  import { onMount } from "svelte";

  type Props = {
    modId: string;
    showVersion?: boolean;
  };

  let { modId, showVersion = true }: Props = $props();

  let mod: ModEntry = $state({
    id: "",
    titleSlug: "",
    displayTitle: "",
    authorSlug: "",
    displayAuthor: "",
    description: "",
    versions: { "0.0.1": "" },
    configFiles: [],
    dependencies: [],
  });

  let title = $derived(mod.displayTitle);
  let author = $derived(mod.displayAuthor);
  let version = $derived(getNewestModVersionSync(mod.versions));

  onMount(async () => {
    mod = await getMod(modId);
  });
</script>

<div class="flex overflow-hidden">
  <div class="pl-3 overflow-hidden text-left shrink">
    <div class="flex gap-2">
      <a
        class="font-medium text-white truncate shrink hover:underline"
        href={communityUrl(`${mod.authorSlug}/${mod.titleSlug}`)}
        target="_blank"
        rel="noopener noreferrer"
      >
        {formatModName(title)}</a
      >

      {#if showVersion && version !== null}
        <span class="text-primary-400 shrink-0">
          {version}
        </span>
      {/if}
    </div>

    {#if author !== null}
      <a
        class="truncate text-primary-400 hover:underline"
        href={communityUrl(communityUrl(mod.authorSlug))}
        target="_blank"
      >
        {author}
      </a>
    {/if}
  </div>
</div>
