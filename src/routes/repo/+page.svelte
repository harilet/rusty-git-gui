<script lang="ts">
  import Titlebar from "$lib/titlebar.svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";

  import * as Diff2Html from "diff2html";
  import "diff2html/bundles/css/diff2html.min.css";
  import { ColorSchemeType } from "diff2html/lib/types";
  import Add from "$lib/add.svelte";
  import type { MouseEventHandler } from "svelte/elements";
  import Remove from "$lib/remove.svelte";
  import Diff from "$lib/diff.svelte";

  let location: String | null;
  let branchList: any[] = [];

  $: selectedBranch = "";
  $: commitMessages = [];
  $: mainTab = "graph";
  $: unstagedFiles = [];
  $: stagedFiles = [];
  $: changes = "";
  $: makeCommitMessage = "";

  let configuration: Diff2Html.Diff2HtmlConfig = {
    drawFileList: false,
    matching: "words",
    colorScheme: ColorSchemeType.DARK,
    outputFormat: "side-by-side",
    maxLineSizeInBlockForComparison: 10,
  };

  function drawDiff() {
    return Diff2Html.html(changes, configuration);
  }

  onMount(() => {
    const hasLocation = $page.url.searchParams.has("location");
    if (hasLocation) {
      location = $page.url.searchParams.get("location");
      invoke("get_branches", { repoLocation: location }).then(function (
        data: any
      ) {
        for (let i = 0; i < data["branches"].length; i++) {
          branchList = [...branchList, data["branches"][i][0]];
        }
        selectBranch(data["default"]);
      });
    }

    listen("changes", function (data) {
      changes = changes + (data.payload as string);
    });
  });

  function selectBranch(branchName: string) {
    selectedBranch = branchName;
    invoke("checkout_branch", {
      repoLocation: location,
      branchName: branchName,
    });
    invoke("get_commit", {
      repoLocation: location,
      branchName: branchName,
    }).then(function (data: any) {
      commitMessages = data;
    });
  }

  function getFileChanges(commit_id: string) {
    changes = "";
    invoke("get_file_changes", {
      repoLocation: location,
      commitId: commit_id,
    });
  }

  function tabChange(tabName: string) {
    changes = "";
    mainTab = tabName;
    if (tabName == "commit") {
      getChangeFiles();
    } else if (tabName == "graph") {
      invoke("get_commit", {
        repoLocation: location,
        branchName: selectedBranch,
      }).then(function (data: any) {
        commitMessages = data;
      });
    }
  }

  function getChangeFiles() {
    invoke("get_unstaged_files", {
      repoLocation: location,
    }).then(function (data: any) {
      unstagedFiles = data;
    });

    invoke("get_staged_files", {
      repoLocation: location,
    }).then(function (data: any) {
      stagedFiles = data;
    });
  }

  function getChanges(type: string, path: string) {
    if (type == "unstaged") {
      changes = "";
      invoke("get_unstaged_changes", {
        repoLocation: location,
        path: path,
      });
    } else if (type == "staged") {
      changes = "";
      invoke("get_staged_changes", {
        repoLocation: location,
        path: path,
      });
    }
  }

  function addFile(e: any, path: string) {
    e.stopPropagation();
    invoke("add_file_index", {
      repoLocation: location,
      path: path,
    }).then(function () {
      getChangeFiles();
    });
  }

  function removeFile(e: any, path: string) {
    e.stopPropagation();
    invoke("remove_file_index", {
      repoLocation: location,
      path: path,
    }).then(function () {
      getChangeFiles();
    });
  }

  function makeCommit() {
    invoke("make_commit", {
      repoLocation: location,
      message: makeCommitMessage,
    }).then(function () {
      getChangeFiles();
    });
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>
<main class="container overflow-auto-style">
  <div class="branch-area overflow-auto-style">
    {#each branchList as branch}
      <button
        on:dblclick={() => selectBranch(branch)}
        class="branch-name {selectedBranch == branch ? 'branch-selected' : ''}"
      >
        {branch}
      </button>
    {/each}
  </div>
  <div class="header">
    <div class="header-buttons">
      <button
        class="item"
        on:click={(_) => tabChange("graph")}
        style="color:{mainTab == 'graph'
          ? 'var(--primary-color)'
          : 'var(--text-color)'}"
      >
        Graph
      </button>
      <button
        class="item"
        on:click={(_) => tabChange("commit")}
        style="color:{mainTab == 'commit'
          ? 'var(--primary-color)'
          : 'var(--text-color)'}"
      >
        Commit
      </button>
    </div>
    <div class="main-area">
      {#if mainTab == "graph"}
        <div style="display: flex;">
          <div class="overflow-auto-style" style="width: 50%;">
            {#each commitMessages as commitMessage}
              <button
                class="commit-message"
                on:click={(_) => getFileChanges(commitMessage[0])}
              >
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
              </button>
            {/each}
          </div>
          <div style="width: 50%;">
            {@html drawDiff()}
          </div>
        </div>
      {/if}
      {#if mainTab == "commit"}
        <div class="main-commit-area overflow-auto-style">
          <div class="file-changes overflow-auto-style">
            
                <!-- {@html drawDiff()} -->
                 <Diff diff={changes}/>
              
          </div>
          <div class="change-files">
            <div class="unstaged-change-files">
              <h2>Unstaged</h2>
              {#each unstagedFiles as files}
                <button
                  on:click={(_) => getChanges("unstaged", files)}
                  class="commit-file-path"
                >
                  {files}
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div
                    on:click={function (e) {
                      addFile(e, files);
                    }}
                  >
                    <Add color="var(--text-color)" />
                  </div>
                </button>
              {/each}
            </div>
            <div class="staged-change-files">
              <h2>Staged</h2>
              {#each stagedFiles as files}
                <button
                  on:click={(_) => getChanges("staged", files)}
                  class="commit-file-path"
                >
                  {files}
                  <!-- svelte-ignore a11y_click_events_have_key_events -->
                  <!-- svelte-ignore a11y_no_static_element_interactions -->
                  <div
                    on:click={function (e) {
                      removeFile(e, files);
                    }}
                  >
                    <Remove color="var(--text-color)" />
                  </div>
                </button>
              {/each}
            </div>
          </div>

          <div style="height: 70px;">
            <textarea bind:value={makeCommitMessage}> </textarea>
            <button on:click={makeCommit}> commit </button>
          </div>
        </div>
      {/if}
    </div>
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
    height: calc(100%);
    resize: horizontal;
    overflow: auto;
    width: 250px;
    border-right: 1px solid black;
    display: block;
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
    text-overflow: ellipsis;
    overflow: hidden;
  }

  .branch-name:hover {
    background: #ffffff40;
  }

  .branch-selected {
    background: #ffffff40;
  }

  .main-area {
    width: -webkit-fill-available;
    height: calc(100% - 40px);
  }

  .commit-message {
    margin: 5px;
    padding: 5px;
    width: -webkit-fill-available;
    text-align: start;
    background: transparent;
    color: var(--text-color);
    border: none;
    border-radius: 4px;
  }

  .commit-message:hover {
    background: #ffffff40;
  }

  .commit-message-info {
    display: flex;
    div {
      margin: 5px;
    }
  }

  .header {
    display: flex;
    flex-direction: column;
    width: -webkit-fill-available;
    .header-buttons {
      display: flex;
      margin-bottom: 20px;
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

  .main-commit-area {
    height: 100%;
    width: 100%;
  }

  .commit-file-path {
    padding: 6px;
    cursor: pointer;
    border-radius: 4px;
    background: transparent;
    color: var(--text-color);
    border: none;
    width: -webkit-fill-available;
    text-align: start;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .commit-file-path:hover {
    background: #ffffff40;
  }

  .change-files {
    display: flex;
    height: 35%;
  }

  .unstaged-change-files {
    width: 50%;
    height: 30vh;
    overflow: auto;
  }
  .staged-change-files {
    width: 50%;
    height: 30vh;
    overflow: auto;
  }

  .file-changes{
    height: calc(65% - 70px);
  }
</style>
