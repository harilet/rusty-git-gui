<script lang="ts">
  import Titlebar from "../lib/ui-components/titlebar.svelte";
  import Add from "../lib/svelte-svg/add.svelte";
  import Computer from "../lib/svelte-svg/computer.svelte";
  import Download from "../lib/svelte-svg/download.svelte";
  import FolderOpen from "../lib/svelte-svg/folder_open.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import Button from "$lib/ui-components/button.svelte";
  import Input from "$lib/ui-components/input.svelte";
  import { onMount } from "svelte";
  import ContextMenu from "$lib/ui-components/contextMenu.svelte";
  import DialogBox from "$lib/ui-components/dialogBox.svelte";

  $: pageMode = "project";

  $: repoUrl = "";
  $: repoLocation = "";
  $: repoName = "";
  $: inProgress = false;

  $: recentProjects = [];

  onMount(() => {
    invoke("get_projects_list").then(function (data: any) {
      recentProjects = data.reverse();
    });
  });

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

  function creatRepo() {
    console.log(repoLocation);
    invoke("create_repo_window", { repoLocation: repoLocation }).then(()=>{
      invoke("open_repo_window", { repoLocation: repoLocation });
    });
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>

<main class="container">
  <div class="header">
    <div class="header-buttons">
      <Button
        isSelected={pageMode == "project"}
        onClick={(_: any) => changeType("project")}
        buttonType="select"
      >
        <div class="item">
          <Computer
            color={pageMode == "project"
              ? "var(--primary-color)"
              : "var(--on-background-color)"}
          />
          Projects
        </div>
      </Button>

      <Button
        onClick={(_: any) => changeType("clone")}
        isSelected={pageMode == "clone"}
        buttonType="select"
      >
        <div class="item">
          <Download
            color={pageMode == "clone"
              ? "var(--primary-color)"
              : "var(--on-background-color)"}
          />
          Clone
        </div>
      </Button>
      <Button
        onClick={(_: any) => changeType("open")}
        isSelected={pageMode == "open"}
        buttonType="select"
      >
        <div class="item">
          <FolderOpen
            color={pageMode == "open"
              ? "var(--primary-color)"
              : "var(--on-background-color)"}
          />
          Open
        </div>
      </Button>
      <Button
        onClick={(_: any) => changeType("create")}
        isSelected={pageMode == "create"}
        buttonType="select"
      >
        <div class="item">
          <Add
            color={pageMode == "create"
              ? "var(--primary-color)"
              : "var(--on-background-color)"}
          />
          Create
        </div>
      </Button>
    </div>
    {#if inProgress}
      <div class="loader"></div>
    {/if}
  </div>
  <div class="m-4 w-3/6">
    {#if pageMode == "project"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Recent Projects</div>
        {#each recentProjects as project}
          <Button
            onClick={(_: any) => {
              repoLocation = project;
              openRepo();
            }}
            buttonType="secondary"
          >
            {project}
          </Button>
        {/each}
      </div>
    {:else if pageMode == "clone"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Clone Project</div>

        <div class="flex flex-col space-y-1.5">
          <lable for="name">URL</lable>
          <Input bind:value={repoUrl} placeholder="Remote project URL" />
        </div>
        <div class="flex flex-col space-y-1.5">
          <lable for="name">Location</lable>
          <div class="flex items-center space-x-2 w-full">
            <div style="width: calc(100% - 125px);">
            <Input
              bind:value={repoLocation}
              placeholder="Location of the project"
            />
          </div>
            <Button buttonType="secondary" onClick={openFileSelector}
              >Open Exploer</Button
            >
          </div>
        </div>
        <div class="flex flex-col space-y-1.5">
          <lable for="name">Name</lable>
          <Input bind:value={repoName} placeholder="Name of your project" />
        </div>
        <Button onClick={cloneProject}>Clone</Button>
      </div>
    {:else if pageMode == "open"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Open Projects</div>
        <div class="flex items-center justify-between">
          <div style="width: calc(100% - 125px);">
            <Input bind:value={repoLocation} placeholder="repo location" />
          </div>

          <Button buttonType="secondary" onClick={openFileSelector}
            >Open Exploer</Button
          >
        </div>
        <Button onClick={openRepo} disabled={repoLocation == ""}>Open</Button>
      </div>
    {:else if pageMode == "create"}
      <div class="grid w-full items-center gap-4">
        <div class="block font-semibold">Create New Repo</div>
        <div class="flex items-center justify-between">
          <div style="width: calc(100% - 125px);">
            <Input bind:value={repoLocation} placeholder="repo location" />
          </div>

          <Button buttonType="secondary" onClick={openFileSelector}
            >Open Exploer</Button
          >
          
        </div>
        <div>
          <Input bind:value={repoName} placeholder="Name" />
        </div>
        <Button onClick={creatRepo} disabled={repoLocation == ""}>Create</Button>
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
    color: var(--on-background-color);
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
