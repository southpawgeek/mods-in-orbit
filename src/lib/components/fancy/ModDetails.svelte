<script lang="ts">
  import ModCardList from "./ModCardList.svelte";
  import DependenciesDialog from "./DependenciesDialog.svelte";
  import Network from "lucide-svelte/icons/network";
  import Download from "lucide-svelte/icons/download";
  import Check from "lucide-svelte/icons/check";
  import {
    communityUrl,
    formatModName,
    getNewestModVersionSync,
  } from "$lib/util";
  import type { ModEntry, Versions } from "$lib/types";
  import { onMount, type Snippet } from "svelte";
  import * as semver from "semver";
  import {
    checkIfModInstalled,
    getNewestModVersion,
    triggerInstallMod,
  } from "$lib/api/commands";
  import { listen } from "@tauri-apps/api/event";
  import { emit } from "@tauri-apps/api/event";
  import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
  import { Dialog } from "bits-ui";
  import Header from "./Header.svelte";

  type Props = {
    mod: ModEntry;
    onclose: () => void;
    children?: Snippet;
  };

  let { mod, onclose, children }: Props = $props();

  let newestVersion: string = $derived(getNewestModVersionSync(mod.versions));

  let dependenciesOpen: boolean = $state(false);

  let isInstalled: boolean = $state(false);

  onMount(async () => {
    newestVersion = await getNewestModVersion(mod.id);
    isInstalled = await checkIfModInstalled(mod.id);

    setInterval(async () => {
      // THIS IS A PATCH!! PLEASE FIX THIS BEFORE YOU RELEASE!! THIS IS SO STUPID IF YOU HAVE LIKE 100 MODS DOING THIS!!
      isInstalled = await checkIfModInstalled(mod.id);
    }, 250);
  });

  //   listen<ModEntry>("download-started", (event) => {
  // if (event.payload.id == mod.id) {
  //   console.log(`received download-started for mod ${event.payload.id}`);
  // }
  //   });
  //
  //   listen<ModEntry>("download-finished", (event) => {
  // if (event.payload.id == mod.id && !isInstalled) {
  //   console.log(`received download-finished for mod ${event.payload.id}`);
  //   isInstalled = true;
  // }
  //   });
  //
  //   $effect(() => {
  // const appWebview = getCurrentWebviewWindow();
  // appWebview.emit("check-mod-installed", mod);
  //   });
  //
  //   type CheckModInstalledResponse = {
  // id: string;
  // isInstalled: boolean;
  //   };
  //
  //   listen<CheckModInstalledResponse>("check-mod-installed-response", (event) => {
  // if (event.payload.id == mod.id) {
  //   isInstalled = event.payload.isInstalled;
  // }
  //   });
  //
  //   listen<string[]>("sync-downloaded-mods", (event) => {
  // if (event.payload.includes(mod.id)) {
  //   isInstalled = true;
  // } else {
  //   isInstalled = false;
  // }
  //   });
</script>

<div class="relative flex min-h-0 min-w-72 w-[40%] flex-col overflow-hidden">
  <div class="fancy-headered-content">
    <Header
      text={formatModName(mod.displayTitle)}
      position="left"
    />

    <p class="mt-0 text-xl grow">
      {#if mod.description !== null}
        <span class="text-primary-50">{mod.description}</span>
        <br />
      {/if}
      {#if mod.displayAuthor}
        <span class="mt-4 text-primary-500"
          >Author: {mod.displayAuthor}
          {#if mod.authorSlug}<a
              href={communityUrl(mod.authorSlug)}
              target="_blank"
              title="View author on GitHub"
              ><i class="fa-brands fa-github"></i></a
            >{/if}</span
        >
        <br />
      {/if}
      {#if newestVersion}
        <span class="text-primary-500">{newestVersion}</span>
        <br />
      {/if}
    </p>

    {#if mod.authorSlug && mod.titleSlug}
      <button
        class="flex items-center py-1 pl-3 pr-1 mt-1 text-white rounded-md group bg-primary-600 hover:bg-primary-500"
      >
        <a
          href={communityUrl(`${mod.authorSlug}/${mod.titleSlug}`)}
          target="_blank"><i class="fa-brands fa-github mr-2"></i> View on GitHub</a
        >
      </button>{/if}

    {#if mod.dependencies !== null && mod.dependencies.length > -1}
      <button
        class="flex items-center py-1 pl-3 pr-1 mt-1 text-white rounded-md group bg-primary-600 hover:bg-primary-500"
        onclick={() => (dependenciesOpen = true)}
      >
        <Network class="mr-2 text-lg" />
        Dependencies
        <div
          class="bg-primary-500 group-hover:bg-primary-400 ml-auto rounded-md px-3 py-0.5 text-sm"
        >
          {mod.dependencies.length}
        </div>
      </button>
    {/if}

    {#if !isInstalled}
      <button
        class="flex items-center py-1 pl-3 pr-1 mt-1 text-white rounded-md group bg-accent-600 hover:bg-accent-500"
        onclick={async () => await triggerInstallMod(mod.id)}
      >
        <Download class="mr-2 text-lg" />
        Install
      </button>
    {:else}
      <button
        class="flex items-center py-1 pl-3 pr-1 mt-1 text-white rounded-md group bg-primary-600 hover:bg-primary-500"
        onclick={async () => console.log("uninstall unimplemented")}
      >
        <Check class="mr-2 text-lg" />
        Already Installed
      </button>
    {/if}

    <DependenciesDialog
      title={mod.displayTitle}
      bind:open={dependenciesOpen}
    >
      {#if mod.dependencies}
        <ModCardList
          ids={mod.dependencies}
          class="mt-4"
        />
      {/if}
    </DependenciesDialog>
  </div>
</div>
