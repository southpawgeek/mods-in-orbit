<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  // import { RefreshCw } from "lucide-svelte";
  import ModList from "$lib/components/fancy/ModList.svelte";
  import { getMods, greet } from "$lib/api/commands";
  import { onMount } from "svelte";
  import type { ModEntry } from "$lib/types";
  import ModDetails from "$lib/components/fancy/ModDetails.svelte";
  import LaunchGame from "$lib/components/fancy/LaunchGame.svelte";
  import Header from "$lib/components/fancy/Header.svelte";
  import DefaultSidebar from "$lib/components/fancy/DefaultSidebar.svelte";
  import ModTabs from "$lib/components/fancy/ModTabs.svelte";

  let name = $state("");
  let greetMsg = $state("");

  let mods: ModEntry[] = $state([]);
  let selectedMod: ModEntry | null = $state(null);
  let maxCount: number = $state(20);
  let modFilter: "all" | "installed" | "not-installed" = $state("all");

  onMount(async () => {
    mods = await getMods();
  });
</script>

<div class="main-wrapper">
  <div class="flex min-h-0 flex-1 overflow-hidden">
    <div
      class="flex min-h-0 flex-1 flex-col overflow-hidden w-[60%] justify-start gap-0"
    >
      <div class="fancy-headered-content">
        <Header
          text="Mods in Orbit"
          position="left"
        />
        <LaunchGame />
        <div class="flex flex-col w-full overflow-hidden grow">
          <ModTabs bind:value={modFilter} />
          <ModList
            {mods}
            filter={modFilter}
            bind:maxCount
            bind:selected={selectedMod}
          />
        </div>
      </div>
    </div>

    {#if selectedMod}
      <ModDetails
        mod={selectedMod}
        onclose={() => (selectedMod = null)}
      ></ModDetails>
    {:else}
      <DefaultSidebar />
    {/if}
  </div>
</div>
