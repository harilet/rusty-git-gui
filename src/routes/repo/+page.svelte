<script lang="ts">
  import Titlebar from "$lib/ui-components/titlebar.svelte";
  import { page } from "$app/stores";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { listen } from "@tauri-apps/api/event";
  import Add from "$lib/svelte-svg/add.svelte";
  import Remove from "$lib/svelte-svg/remove.svelte";
  import Diff from "$lib/ui-components/diff.svelte";
  import ContextMenu from "$lib/ui-components/contextMenu.svelte";
  import Button from "$lib/ui-components/button.svelte";
  import DialogBox from "$lib/ui-components/dialogBox.svelte";

  let location: String | null;
  let branchList: any[] = [];

  $: selectedBranch = "";
  $: commitMessages = [];
  $: mainTab = "graph";
  $: unstagedFiles = [];
  $: stagedFiles = [];
  $: changes = "";
  $: makeCommitMessage = "";
  $: force = false;
  $: newBranchName = "";

  let newBranchDialog: HTMLDialogElement;
  let renameBranchDialog: HTMLDialogElement;
  let deleteBranchDialog: HTMLDialogElement;

  onMount(() => {
    const hasLocation = $page.url.searchParams.has("location");
    if (hasLocation) {
      location = $page.url.searchParams.get("location");
      getAllBranchs();
    }

    listen("changes", function (data) {
      changes = changes + (data.payload as string);
    });
  });

  function getAllBranchs() {
    invoke("get_branches", { repoLocation: location }).then(function (
      data: any,
    ) {
      for (let i = 0; i < data["branches"].length; i++) {
        branchList = [...branchList, data["branches"][i][0]];
      }
      selectBranch(data["default"]);
    });
  }

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

  function removeFileFromIndex(e: any, path: string) {
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

  function creatBranch() {
    invoke("new_branch", {
      repoLocation: location,
      force: force,
      newBranchName: newBranchName,
      fromBranchName: selectedBranch,
    }).then(function () {
      branchList = [];
      getAllBranchs();
      newBranchDialog.close();
    });
  }

  function renameBranch(branch: string) {
    invoke("rename_branch", {
      repoLocation: location,
      branchName: branch,
      newBranchName: newBranchName,
      force: force,
    }).then(function () {
      branchList = [];
      getAllBranchs();
    });
  }

  function deleteBranch(branch: string) {
    invoke("delete_branch", {
      repoLocation: location,
      branchName: branch,
    }).then(function () {
      branchList = [];
      getAllBranchs();
    });
  }

  function branchContextMenuHandler(e: any, i: any) {
    switch (i) {
      case "rename":
        renameBranchDialog.showModal();
        break;
      case "delete":
        deleteBranchDialog.showModal();
        break;
    }
  }

  function unstagedContextMenuHandler(i: any, files: any) {
    switch (i) {
      case "Open":
        open(files);
        break;
      case "Add":
        break;
      case "Remove":
        break;
      case "discard":
        break;
    }
  }

  function stagedContextMenuHandler(i: any, files: any) {
    switch (i) {
      case "Open":
        break;
      case "Add":
        break;
      case "Remove":
        break;
      case "discard":
        break;
    }
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar />
</div>

<DialogBox bind:dialog={newBranchDialog}>
  <div class="flex flex-col">
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">New Branch Name</label>
      <input bind:value={newBranchName} placeholder="Name" />
    </div>
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">From Branch</label>
      <input bind:value={selectedBranch} placeholder="Branch" />
    </div>
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">Force</label>
      <input type="checkbox" bind:checked={force} />
    </div>
    <Button buttonType="secondary" onClick={creatBranch}>Create</Button>
  </div>
</DialogBox>

<main class="container overflow-auto-style">
  <div class="flex w-full">
    <div class="branch-area overflow-auto-style flex flex-col w-1/6">
      <div style="margin: 0px 5px; --width: 100%;">
        <Button
          onClick={(_: any) => {
            newBranchDialog.showModal();
          }}
        >
          New Branch
        </Button>
      </div>

      {#each branchList as branch}
        <ContextMenu
          items={["rename", "delete"]}
          onClick={branchContextMenuHandler}
        >
          <button
            on:dblclick={() => selectBranch(branch)}
            class="branch-name {selectedBranch == branch
              ? 'branch-selected'
              : ''}"
          >
            {branch}
          </button>
        </ContextMenu>

        <DialogBox bind:dialog={renameBranchDialog}>
          <div class="flex flex-col">
            <div class="flex w-full max-w-sm flex-col gap-1.5">
              <label for="email">Rename</label>
              <input bind:value={newBranchName} placeholder="Rename" />
            </div>
            <div class="flex w-full max-w-sm flex-col gap-1.5">
              <label for="email">Force</label>
              <input type="checkbox" bind:checked={force} />
            </div>
            <Button
              buttonType="secondary"
              onClick={(_: any) => renameBranch(branch)}>Rename</Button
            >
          </div>
        </DialogBox>

        <DialogBox bind:dialog={deleteBranchDialog}>
          <div class="flex flex-col">
            <Button
              buttonType="secondary"
              onClick={(_: any) => deleteBranch(branch)}>Delete</Button
            >
          </div>
        </DialogBox>
      {/each}
    </div>
    <div class="w-5/6">
      <div class="header-buttons h-1/10">
        <button
          class="item"
          on:click={(_) => tabChange("graph")}
          style="color:{mainTab == 'graph'
            ? 'var(--primary-color)'
            : 'var(----on-background-colorolor)'}"
        >
          Graph
        </button>
        <button
          class="item"
          on:click={(_) => tabChange("commit")}
          style="color:{mainTab == 'commit'
            ? 'var(--primary-color)'
            : 'var(----on-background-colorackground-colorolor)'}"
        >
          Commit
        </button>
      </div>
      <div class="main-area h-9/10">
        {#if mainTab == "graph"}
          <div class="graph-area">
            <div class="overflow-auto-style commit-message-area">
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
            <div class="commit-changes">
              <Diff diff={changes} />
            </div>
          </div>
        {/if}
        {#if mainTab == "commit"}
          <div class="h-full flex flex-col">
            <div class="h-full overflow-auto-style grow-2">
              <Diff diff={changes} />
            </div>
            <div class="grow flex">
              <div class="w-1/2">
                <h2>Unstaged</h2>
                <div class="unstaged-change-files">
                  {#each unstagedFiles as files}
                    <ContextMenu
                      items={["Open", "Add", "Remove", "discard"]}
                      onClick={function (e: any, i: any) {
                        unstagedContextMenuHandler(i, files);
                      }}
                    >
                      <button
                        on:click={(_) => getChanges("unstaged", files)}
                        class=" commit-file-path"
                      >
                        {files}
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <div
                          on:click={function (e) {
                            addFile(e, files);
                          }}
                        >
                          <Add color="var(--on-background-color)" />
                        </div>
                      </button>
                    </ContextMenu>
                  {/each}
                </div>
              </div>
              <div class="w-1/2">
                <h2>Staged</h2>
                <div class="staged-change-files">
                  {#each stagedFiles as files}
                    <ContextMenu
                      items={["Open", "Add", "Remove", "discard"]}
                      onClick={function (e: any, i: any) {
                        stagedContextMenuHandler(i, files);
                      }}
                    >
                      <button
                        on:click={(_) => getChanges("staged", files)}
                        class="commit-file-path"
                      >
                        {files}
                        <!-- svelte-ignore a11y_click_events_have_key_events -->
                        <!-- svelte-ignore a11y_no_static_element_interactions -->
                        <div
                          on:click={function (e) {
                            removeFileFromIndex(e, files);
                          }}
                        >
                          <Remove color="var(--on-background-color)" />
                        </div>
                      </button>
                    </ContextMenu>
                  {/each}
                </div>
              </div>
              <div class="change-files"></div>
            </div>
            <div class="grow-0">
              <div class="grid w-full gap-2">
                <textarea placeholder="Type your message here."></textarea>
                <Button buttonType="secondary" onClick={makeCommit}
                  >commit</Button
                >
              </div>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</main>

<style>
  .app-bar {
    width: 100%;
    height: 30px;
  }

  .graph-area {
    display: flex;
    height: 100%;
  }

  .commit-changes {
    width: 50%;
    height: 100%;
    overflow: auto;
  }

  .commit-message-area {
    width: 50%;
    height: 100%;
  }

  .container {
    height: calc(100% - 30px);
    width: 100%;
    display: flex;
    padding: 0px;
    margin: 0px;
    max-width: none;
  }

  .branch-name {
    padding: 10px 5px;
    margin: 0px 5px;
    border-radius: 5px;
    cursor: pointer;
    border: none;
    width: -webkit-fill-available;
    background: none;
    color: var(----on-background-colorackground-colorolor);
    text-align: start;
    text-overflow: ellipsis;
    overflow: hidden;
    display: flex;
    align-items: center;
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
    color: var(----on-background-colorolor);
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

  .header-buttons {
    display: flex;
  }
  .item {
    width: 70px;
    display: flex;
    flex-direction: column;
    align-items: center;
    background: transparent;
    border: none;
    color: var(----on-background-colorackground-colorolor);
    cursor: pointer;
  }

  .commit-file-path {
    padding: 6px;
    cursor: pointer;
    border-radius: 4px;
    background: transparent;
    color: var(----on-background-colorolor);
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
    height: 30vh;
    overflow: auto;
  }

  .staged-change-files {
    height: 30vh;
    overflow: auto;
  }
</style>
