<script lang="ts">
  import ModEntry from "$lib/components/ModListEntry.svelte";
  import { getMods } from "$lib/api/commands";
  import { onMount } from "svelte";
  import type { ModEntry as Mod } from "$lib/types";
  import { listen } from "@tauri-apps/api/event";

  const itemHeight = 66;

  type Props = {
    mods: Mod[];
    filter?: "all" | "installed" | "not-installed";
    maxCount: Number;
    selected: Mod | null;
  };

  let {
    mods,
    filter = "all",
    maxCount = $bindable(20),
    selected = $bindable(null),
  }: Props = $props();

  $effect(() => {
    filter; // track filter changes
    selected = null;
  });

  function selectMod(mod: Mod) {
    if (selected === null || selected.id !== mod.id) {
      selected = mod;
    } else {
      selected = null;
    }
  }

  // listen<string[]>("sync-downloaded-mods", (event) => {
  // console.log("syncing downloaded mods");
  // });
</script>

<div class="@container bg-primary-800 mx-3 mb-2.5 mt-1 py-0.5 rounded-xl">
  {#each mods as mod}
    <ModEntry
      {mod}
      {filter}
      isSelected={selected?.id == mod.id}
      onclick={(event: MouseEvent) => {
        event.preventDefault();
        selectMod(mod);
      }}
      onkeydown={(event: KeyboardEvent) => {
        event.preventDefault();
        selectMod(mod);
      }}
    />
  {/each}
</div>
