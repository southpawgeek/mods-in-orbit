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

  let name = $state("");
  let greetMsg = $state("");

  let mods: ModEntry[] = $state([]);
  let selectedMod: ModEntry | null = $state(null);
  let maxCount: number = $state(20);

  onMount(async () => {
    mods = await getMods();
  });
</script>

<div class="main-wrapper">
  <div class="flex overflow-hidden grow">
    <div class="flex flex-col overflow-hidden grow w-[60%] justify-start gap-0">
      <div class="fancy-headered-content">
        <Header
          text="Mods in Orbit"
          position="left"
        />
        <LaunchGame />
        <div class="flex flex-col w-full overflow-hidden grow">
          <ModList
            {mods}
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
    {/if}
  </div>
</div>
