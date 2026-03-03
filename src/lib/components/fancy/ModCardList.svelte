<script lang="ts">
  import type { ClassValue } from "svelte/elements";
  import ModCard from "./ModCard.svelte";

  type Props = {
    ids: string[];
    showVersion?: boolean;
    class?: ClassValue;
  };

  let { ids, showVersion = true, class: classProp }: Props = $props();
</script>

<div class={[classProp, "grid gap-3 overflow-y-auto"]}>
  {#each ids as modId}
    <ModCard {modId} {showVersion} />
  {/each}
</div>

<style>
  .list {
    grid-template-columns: repeat(auto-fill, minmax(17rem, 1fr));
  }
</style>
