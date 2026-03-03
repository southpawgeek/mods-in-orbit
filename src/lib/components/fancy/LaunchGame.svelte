<script lang="ts">
  import Play from "lucide-svelte/icons/play";
  import { launchGame } from "$lib/api/commands";
  import type { ClassValue } from "svelte/elements";

  type Props = {
    class?: ClassValue;
  };

  let { class: classProp }: Props = $props();
</script>

<div class={[classProp, "flex flex-col p-2 mb-0 mx-1 mt-1"]}>
  <button
    class="flex items-center justify-center p-3 font-semibold text-center border-none rounded-xl grow text-accent-500 bg-primary-800 hover:bg-primary-700"
    onclick={launchGame}
  >
    <Play class="mr-1" />
    Launch Game
  </button>
</div>
