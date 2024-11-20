<script lang="ts">
  import Titlebar from "$lib/titlebar.svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import TreeView from "$lib/TreeView.svelte";

  let location: String | null;
  let branchList: any[] = [];

  $: selectedBranch = "";
  $: commitMessages = [];

  onMount(() => {
    const hasLocation = $page.url.searchParams.has("location");
    if (hasLocation) {
      location = $page.url.searchParams.get("location");
      invoke("get_branches", { repoLocation: location }).then(function (
        data: any
      ) {
        console.log(data);
        for (let i = 0; i < data["branches"].length - 1; i++) {
          branchList = [...branchList, data["branches"][i][0]];
        }
        selectBranch(data["default"]);
      });

      invoke("get_commit", { repoLocation: location }).then(function (
        data: any
      ) {
        console.log(data);
        commitMessages = data;
      });
    }
  });

  function selectBranch(branchName: string) {
    selectedBranch = branchName;
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>
<main class="container">
  <div class="branch-area">
    {#each branchList as branch}
      <button
        on:dblclick={() => selectBranch(branch)}
        class="branch-name {selectedBranch == branch ? 'branch-selected' : ''}"
      >
        {branch}
      </button>
    {/each}
  </div>
  <div class="commit-messages-area">
    {#each commitMessages as commitMessage}
      <div class="commit-message">
        <div>
          {commitMessage[3]}
        </div>
        <div class="commit-message-info">
          <div>
            {commitMessage[1]}
          </div>
          <div>
            {commitMessage[2]}
          </div>
          <div>
            {commitMessage[0]}
          </div>
        </div>
      </div>
    {/each}
  </div>
</main>

<style>
  .app-bar {
    width: 100%;
    height: 30px;
  }

  .container {
    height: calc(100% - 30px);
    width: 100%;
    display: flex;
  }

  .branch-area {
    overflow: auto;
    height: calc(100%);
    width: 243px;
  }

  .branch-name {
    padding: 10px 5px;
    margin: 0px 5px;
    border-radius: 5px;
    cursor: pointer;
    border: none;
    width: -webkit-fill-available;
    background: none;
    color: var(--text-color);
    text-align: start;
  }

  .branch-name:hover {
    background: #ffffff40;
  }

  .branch-selected {
    background: #ffffff40;
  }

  .commit-messages-area {
    width: fit-content;
    overflow: auto;
  }

  .commit-message{
    margin: 5px;
    padding: 5px;
  }

  .commit-message-info{
    display: flex;
    div{
      margin: 5px;
    }
  }
</style>
