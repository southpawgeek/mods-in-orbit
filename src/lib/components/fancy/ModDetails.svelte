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

<div class="relative flex w-[40%] min-w-72 flex-col border px-6 pt-6 pb-4">
  <div class="fancy-headered-content">
    <Header text={formatModName(mod.displayTitle)} />

    <div>
      <a
        class="pr-4 text-3xl font-bold text-left text-white wrap-break-word xl:text-4xl hover:underline"
        href={communityUrl(`${mod.authorSlug}/${mod.titleSlug}`)}
        target="_blank"
      >
        {formatModName(mod.displayTitle)}
      </a>
      {#if mod.displayAuthor}
        <div class="text-xl text-primary-300 xl:text-2xl">
          by {#if mod.authorSlug}<a
              class="hover:underline"
              href={communityUrl(mod.authorSlug)}
              target="_blank"
            >
              {mod.displayAuthor}
            </a>
          {:else}
            {mod.displayAuthor}
          {/if}
        </div>
      {/if}

      {#if newestVersion}
        <div class="text-xl text-primary-300 xl:text-2xl">v{newestVersion}</div>
      {/if}
    </div>

    {#if mod.description !== null}
      <p class="mt-2 text-xl text-primary-300 grow">
        {mod.description}
      </p>
    {/if}
  </div>

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
</div>

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
