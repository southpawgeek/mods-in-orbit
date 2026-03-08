<script lang="ts">
  export type FilterTab = "all" | "installed" | "not-installed";

  type Props = {
    value?: FilterTab;
  };

  let { value = $bindable("all" as FilterTab) }: Props = $props();

  const tabs: { id: FilterTab; label: string }[] = [
    { id: "all", label: "All" },
    { id: "installed", label: "Installed" },
    { id: "not-installed", label: "Not Installed" },
  ];
</script>

<div class="flex justify-center gap-2">
  {#each tabs as tab}
    <button
      class="rounded-lg px-4 py-2 text-sm font-medium transition-colors {value === tab.id
        ? 'bg-primary-500 text-primary-50'
        : 'bg-primary-700 hover:bg-primary-600 text-primary-50'}"
      onclick={() => (value = tab.id)}
    >
      {tab.label}
    </button>
  {/each}
</div>
