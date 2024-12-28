<script lang="ts">
  import Titlebar from "../lib/titlebar.svelte";
  import Add from "../lib/add.svelte";
  import Computer from "../lib/computer.svelte";
  import Download from "../lib/download.svelte";
  import FolderOpen from "../lib/folder_open.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { Button } from "$lib/components/ui/button";
  import Input from "$lib/components/ui/input/input.svelte";
  import { Label } from "$lib/components/ui/label";

  $: pageMode = "project";

  $: repoUrl = "";
  $: repoLocation = "";
  $: repoName = "";
  $: inProgress = false;

  $: {
    if (repoUrl != "") {
      let urlBreakdown = repoUrl.split("/");
      repoName = urlBreakdown[urlBreakdown.length - 1].split(".")[0];
    }
  }

  $: {
    if (repoLocation != "") {
      let urlBreakdown = repoLocation.split("\\");
      repoName = urlBreakdown[urlBreakdown.length - 1];
    }
  }

  function changeType(value: string) {
    if (!inProgress) {
      pageMode = value;
    }
  }

  async function openFileSelector() {
    const file = await open({
      multiple: false,
      directory: true,
    });
    if (file != null) {
      repoLocation = file;
    }
  }

  function cloneProject() {
    inProgress = true;
    invoke("clone_repo", {
      repoUrl: repoUrl,
      repoLocation: repoLocation,
    });
    listen("clone-complete", function (data) {
      inProgress = false;
      console.log(data);
      openRepo();
    });
  }

  function openRepo() {
    console.log(repoLocation);
    invoke("open_repo_window", { repoLocation: repoLocation });
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>

<main class="container">
  <div class="header">
    <div class="header-buttons">
      <button
        class="item"
        on:click={(_) => changeType("project")}
        style="color:{pageMode == 'project'
          ? 'var(--primary-color)'
          : 'var(--text-color)'}"
      >
        <Computer
          color={pageMode == "project"
            ? "var(--primary-color)"
            : "var(--text-color)"}
        />
        Projects
      </button>
      <button
        class="item"
        on:click={(_) => changeType("clone")}
        style="color:{pageMode == 'clone'
          ? 'var(--primary-color)'
          : 'var(--text-color)'}"
      >
        <Download
          color={pageMode == "clone"
            ? "var(--primary-color)"
            : "var(--text-color)"}
        />
        Clone
      </button>
      <button
        class="item"
        on:click={(_) => changeType("open")}
        style="color:{pageMode == 'open'
          ? 'var(--primary-color)'
          : 'var(--text-color)'}"
      >
        <FolderOpen
          color={pageMode == "open"
            ? "var(--primary-color)"
            : "var(--text-color)"}
        />
        Open
      </button>
      <button
        class="item"
        on:click={(_) => changeType("create")}
        style="color:{pageMode == 'create'
          ? 'var(--primary-color)'
          : 'var(--text-color)'}"
      >
        <Add
          color={pageMode == "create"
            ? "var(--primary-color)"
            : "var(--text-color)"}
        />
        Create
      </button>
    </div>
    {#if inProgress}
      <div class="loader"></div>
    {/if}
  </div>
  <div class="m-4 w-3/6">
    {#if pageMode == "project"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Recent Projects</div>
      </div>
    {:else if pageMode == "clone"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Clone Project</div>

        <div class="flex flex-col space-y-1.5">
          <Label for="name">URL</Label>
          <Input
            bind:value={repoUrl}
            id="name"
            placeholder="Remote project URL"
          />
        </div>
        <div class="flex flex-col space-y-1.5">
          <Label for="name">Location</Label>
          <div class="flex items-center space-x-2 w-full">
            <Input
              bind:value={repoLocation}
              placeholder="Location of the project"
            />
            <Button on:click={openFileSelector} variant="outline"
              >Open Exploer</Button
            >
          </div>
        </div>
        <div class="flex flex-col space-y-1.5">
          <Label for="name">Name</Label>
          <Input
            bind:value={repoName}
            id="name"
            placeholder="Name of your project"
          />
        </div>
        <Button variant="outline" on:click={cloneProject}>Clone</Button>
      </div>
    {:else if pageMode == "open"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Open Projects</div>
        <div class="flex items-center space-x-2">
          <Input bind:value={repoLocation} placeholder="repo location" />
          <Button on:click={openFileSelector} variant="outline"
            >Open Exploer</Button
          >
        </div>
        <Button
          variant="outline"
          on:click={openRepo}
          disabled={repoLocation == ""}>Open</Button
        >
      </div>
    {:else if pageMode == "create"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Create New Repo</div>
      </div>
    {/if}
  </div>
</main>

<style>
  .container {
    width: 100%;
    padding: 0px;
    margin: 0px;
    max-width: none;
  }
  .app-bar {
    width: 100%;
    height: 30px;
  }

  .header {
    display: flex;
    flex-direction: column;
    .header-buttons {
      display: flex;
    }
  }

  .item {
    width: 70px;
    display: flex;
    flex-direction: column;
    align-items: center;
    background: transparent;
    border: none;
    color: var(--text-color);
    cursor: pointer;
  }
  /* HTML: <div class="loader"></div> */
  .loader {
    height: 4px;
    width: 100%;
    --c: no-repeat linear-gradient(var(--primary-color) 0 0);
    background: var(--c), var(--c), transparent;
    background-size: 60% 100%;
    animation: l16 3s infinite;
  }
  @keyframes l16 {
    0% {
      background-position:
        -150% 0,
        -150% 0;
    }
    66% {
      background-position:
        250% 0,
        -150% 0;
    }
    100% {
      background-position:
        250% 0,
        250% 0;
    }
  }
</style>
