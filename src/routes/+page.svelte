<script lang="ts">
  import Titlebar from "../lib/titlebar.svelte";
  import Add from "../lib/add.svelte";
  import Computer from "../lib/computer.svelte";
  import Download from "../lib/download.svelte";
  import FolderOpen from "../lib/folder_open.svelte";
  import { open } from "@tauri-apps/plugin-dialog";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

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
  <div>
    {#if pageMode == "project"}
      <div>
        <h2>Recent Projects</h2>
      </div>
    {:else if pageMode == "clone"}
      <div>
        <h2>Clone Projects</h2>
        <div>
          <div>
            URL<input bind:value={repoUrl} />
          </div>
          <div>
            Location <input bind:value={repoLocation} />
            <button on:click={openFileSelector}>open File</button>
          </div>
          <div>
            Name <input bind:value={repoName} />
          </div>
          <div>
            <button on:click={cloneProject}> Clone </button>
          </div>
        </div>
      </div>
    {:else if pageMode == "open"}
      <div>
        <h2>Open Projects</h2>
        <div>
          Location <input bind:value={repoLocation} />
          <button on:click={openFileSelector}>open FIle</button>
        </div>
        <div>
          <button on:click={openRepo}> open </button>
        </div>
      </div>
    {:else if pageMode == "create"}
      <div>
        <h2>Create Repo</h2>
      </div>
    {/if}
  </div>
</main>

<style>
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
