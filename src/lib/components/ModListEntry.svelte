<script lang="ts">
  import Download from "lucide-svelte/icons/download";
  import X from "lucide-svelte/icons/x";
  import Check from "lucide-svelte/icons/check";
  import type { Versions, ModEntry } from "$lib/types";
  import type { EventHandler, MouseEventHandler } from "svelte/elements";
  import { onMount } from "svelte";
  import * as semver from "semver";
  import {
    checkIfModEnabled,
    checkIfModInstalled,
    disableMod,
    enableMod,
    getNewestModVersion,
    toggleMod,
    triggerInstallMod,
    triggerUninstallMod,
  } from "$lib/api/commands";
  import { listen } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { Switch } from "bits-ui";

  type Props = {
    mod: ModEntry;
    filter?: "all" | "installed" | "not-installed";
    isSelected: boolean;
    onclick: EventHandler<MouseEvent>;
    onkeydown: EventHandler<KeyboardEvent>;
  };

  let { mod, filter = "all", isSelected, onclick, onkeydown }: Props = $props();

  let isInstalled: boolean = $state(false);
  let newestVersion: string = $state("0.0.0");
  let isEnabled: boolean = $state(true);

  let descriptionClasses = $derived(
    isSelected
      ? "text-primary-300"
      : "text-primary-400 group-hover:text-primary-300",
  );

  let isVisible = $derived(
    filter === "all" ||
      (filter === "installed" && isInstalled) ||
      (filter === "not-installed" && !isInstalled),
  );

  onMount(async () => {
    isInstalled = await checkIfModInstalled(mod.id);
    newestVersion = await getNewestModVersion(mod.id);
    isEnabled = await checkIfModEnabled(mod.id);

    setInterval(async () => {
      // THIS IS A PATCH!! PLEASE FIX THIS BEFORE YOU RELEASE!! THIS IS SO STUPID IF YOU HAVE LIKE 100 MODS DOING THIS!!
      isInstalled = await checkIfModInstalled(mod.id);
    }, 250);

    setInterval(async () => {
      // THIS IS A PATCH!! PLEASE FIX THIS BEFORE YOU RELEASE!! THIS IS SO STUPID IF YOU HAVE LIKE 100 MODS DOING THIS!!
      isEnabled = await checkIfModEnabled(mod.id);
    }, 250);
  });

  async function handleInstallClick(event: MouseEvent) {
    event.stopPropagation();
    console.log("installing");
    if ((await triggerInstallMod(mod.id)) == true) {
      isInstalled = true;
    }
  }

  async function handleUninstallClick(event: MouseEvent) {
    event.stopPropagation();
    console.log("uninstalling");
    if ((await triggerUninstallMod(mod.id)) == true) {
      isInstalled = false;
    }
  }

  async function handleEnableClick(event: MouseEvent) {
    event.stopPropagation();
    console.log("enabling");
    await enableMod(mod.id);
  }

  async function handleDisableClick(event: MouseEvent) {
    event.stopPropagation();
    console.log("disabling");
    await disableMod(mod.id);
  }

  async function handleToggleClick(newState: boolean) {
    if (newState !== isEnabled) {
      isEnabled = !isEnabled;
      await toggleMod(mod.id);
    }
  }

  // listen<ModEntry>("download-started", (event) => {
  // console.log("download-started");
  // if (event.payload.id == mod.id) {
  // }
  // });
  //
  // listen<ModEntry>("download-finished", (event) => {
  // console.log("download-finished");
  // if (event.payload.id == mod.id && !isInstalled) {
  // isInstalled = true;
  // }
  // });
  //
  // listen<ModEntry>("uninstall-started", (event) => {
  // console.log("uninstall-started");
  // if (event.payload.id == mod.id) {
  // }
  // });
  //
  // listen<ModEntry>("uninstall-finished", (event) => {
  // console.log("uninstalled-finished");
  // if (event.payload.id == mod.id && isInstalled) {
  // isInstalled = false;
  // }
  //
  // listen<string[]>("sync-downloaded-mods", (event) => {
  // if (event.payload.includes(mod.id)) {
  // isInstalled = true;
  // } else {
  // isInstalled = false;
  // }
  // });
  // });
</script>

<div class="{isVisible ? '' : 'hidden'}">
<button
  class="group flex w-full items-center rounded-lg border p-2 {isSelected
    ? 'border-primary-500 bg-primary-700'
    : 'hover:bg-primary-700 border-transparent'}"
  data-uuid={mod.id}
  {onclick}
  {onkeydown}
>
  <div class="pl-3 pr-2 overflow-hidden text-left shrink grow">
    <div class="flex items-center gap-1 overflow-hidden">
      <div class="pr-1 font-medium text-white truncate shrink">
        {mod.displayTitle} - {newestVersion}
      </div>
      {#if isInstalled}
        <Check class="text-accent-500" />
      {/if}
    </div>
    {#if mod.description !== null}
      <div class="truncate {descriptionClasses}">
        {mod.description}
      </div>
    {/if}
  </div>
  {#if isInstalled}
    <!-- svelte-ignore node_invalid_placement_ssr -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="contents" onclick={(evt) => evt.stopPropagation()}>
      <Switch.Root
        checked={isEnabled ?? true}
        onCheckedChange={handleToggleClick}
        class="group data-[state=checked]:bg-accent-700 data-[state=checked]:hover:bg-accent-600 bg-primary-600 hover:bg-primary-500 mr-1 flex h-6 w-12 shrink-0 rounded-full px-1 py-1"
      >
        <Switch.Thumb
          class="data-[state=checked]:bg-accent-200 bg-primary-300 pointer-events-none h-full w-4 rounded-full transition-transform duration-75 ease-out data-[state=checked]:translate-x-6"
        />
      </Switch.Root>
    </div>
  {/if}
  {#if !isInstalled}
    <!-- svelte-ignore node_invalid_placement_ssr -->
    <button
      class="bg-accent-600 hover:bg-accent-500 disabled:bg-primary-600 disabled:text-primary-300 mt-0.5 mr-0.5 ml-2 rounded-lg p-2.5 align-middle text-2xl text-white group-hover:inline {!isSelected
        ? 'hidden'
        : ''}"
      onclick={handleInstallClick}
    >
      <Download />
    </button>
  {:else}
    <!-- svelte-ignore node_invalid_placement_ssr -->
    <button
      class="bg-red-600 hover:bg-red-500 disabled:bg-primary-600 disabled:text-primary-300 mt-0.5 mr-0.5 ml-2 rounded-lg p-2.5 align-middle text-2xl text-white group-hover:inline {!isSelected
        ? 'hidden'
        : ''}"
      onclick={handleUninstallClick}
    >
      <X />
    </button>
  {/if}
</button>
</div>
