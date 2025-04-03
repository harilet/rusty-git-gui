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
  import Input from "$lib/ui-components/input.svelte";
  import Push from "./push.svelte";
  import BranchItem from "./branchItem.svelte";
  import TabSelect from "$lib/ui-components/tabSelect.svelte";
  import CollapsibleSection from "$lib/ui-components/collapsibleSection.svelte";
  import Pull from "./pull.svelte";
  import BranchList from "./branchList.svelte";
  import ErrorToast from "$lib/ui-components/errorToast.svelte";

  let location: string | null;
  let localBranchList: any[] = [];
  let remoteBranchList: any[] = [];

  $: selectedBranch = "";
  $: commitMessages = [];
  $: mainTab = "graph";
  $: unstagedFiles = [];
  $: stagedFiles = [];
  $: changes = [];
  $: makeCommitMessage = "";
  $: newBranchFrom = selectedBranch;
  $: force = false;
  $: newBranchName = "";
  $: userName = "";
  $: userEmail = "";
  $: remotesList = [];
  $: newRemoteName = "";
  $: newRemoteURL = "";
  $: currentBranchRemoteCommitDelta = [];
  $: error = [""];

  let newBranchDialog: HTMLDialogElement;
  let profileDialog: HTMLDialogElement;
  let newRemoteDialog: HTMLDialogElement;

  onMount(() => {
    const hasLocation = $page.url.searchParams.has("location");
    if (hasLocation) {
      location = $page.url.searchParams.get("location");
      getAllBranchs();
    }

    listen("changes", function (data: any) {
      changes = data.payload;
    });

    listen("error", function (data: any) {
      let message:string=data.payload;
      error =[...error,message];
    });

    invoke("get_config", {
      repoLocation: location,
      keyName: "user.name",
    }).then(function (data: any) {
      userName = data;
    });

    invoke("get_config", {
      repoLocation: location,
      keyName: "user.email",
    }).then(function (data: any) {
      userEmail = data;
    });

    invoke("get_remotes", {
      repoLocation: location,
    }).then((res: any) => {
      remotesList = res;
    });
  });

  function getAllBranchs() {
    invoke("get_branches", { repoLocation: location }).then(function (
      data: any,
    ) {
      let tempLocalBranchList: any[] = [];
      for (let i = 0; i < data["branches"]["local"].length; i++) {
        tempLocalBranchList = [
          ...tempLocalBranchList,
          data["branches"]["local"][i],
        ];
      }
      localBranchList = tempLocalBranchList;
      for (let i = 0; i < data["branches"]["remote"].length; i++) {
        remoteBranchList = [...remoteBranchList, data["branches"]["remote"][i]];
      }
      selectBranch(data["default"]);
    });
  }

  function checkoutBranch(branchName: string) {
    invoke("checkout_branch", {
      repoLocation: location,
      branchName: branchName,
    });
    refetchBranchs();
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

    invoke("get_remote_local_drift", {
      repoLocation: location,
      branchName: branchName,
    }).then(function (diffStat: any) {
      console.log(typeof diffStat);
      currentBranchRemoteCommitDelta = diffStat;
    });
  }

  function getFileChanges(commit_id: string) {
    changes = [];
    invoke("get_file_changes", {
      repoLocation: location,
      commitId: commit_id,
    });
  }

  function tabChange(tabName: string) {
    changes = [];
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
      changes = [];
      invoke("get_unstaged_changes", {
        repoLocation: location,
        path: path,
      });
    } else if (type == "staged") {
      changes = [];
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
      fromBranchName: newBranchFrom,
    }).then(function () {
      refetchBranchs();
      newBranchDialog.close();
    });
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

  function getGitName(locationName: string) {
    let locationSplit = locationName.split("\\");
    return locationName.split("\\")[locationSplit.length - 1];
  }

  function showOptionsDialog() {
    profileDialog.showModal();
  }

  function showNewRemoteDialog() {
    newRemoteDialog.showModal();
  }

  function addNewRemote() {
    invoke("add_remote", {
      repoLocation: location,
      remoteName: newRemoteName,
      remoteUrl: newRemoteURL,
    }).then((_) => {
      invoke("get_remotes", {
        repoLocation: location,
      }).then((res: any) => {
        remotesList = res;
      });
    });
  }

  function refetchBranchs() {
    localBranchList = [];
    remoteBranchList = [];
    getAllBranchs();
  }
</script>

<div data-tauri-drag-region class="app-bar">
  <Titlebar title={location == undefined ? "" : getGitName(location)} />
</div>

  <ErrorToast bind:message={error} />
  

<DialogBox bind:dialog={newBranchDialog}>
  <div class="flex flex-col m-8">
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">New Branch Name</label>
      <Input bind:value={newBranchName} placeholder="Name" />
    </div>
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">From Branch</label>
      <select bind:value={newBranchFrom}>
        {#each localBranchList as branch}
          <option value={branch} selected>{branch}</option>
        {/each}
      </select>
    </div>
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">Force</label>
      <Input type="checkbox" bind:checked={force} />
    </div>
    <Button buttonType="secondary" onClick={creatBranch}>Create</Button>
  </div>
</DialogBox>

<main class="container overflow-auto-style">
  <div class="flex w-full">
    <div class="branch-area overflow-auto-style flex flex-col pl-2 w-64">
      <div style="margin: 0px 5px; --width: 100%;">
        <Button
          onClick={(_: any) => {
            newBranchDialog.showModal();
          }}
        >
          New Branch
        </Button>
      </div>
      <div class="overflow-auto-style flex flex-col">
        <CollapsibleSection title="Local Branch" defaultCollapsed={false}>
          <div class="ml-3 flex flex-col">
            <BranchList
              branchList={localBranchList}
              bind:location
              bind:selectedBranch
              selectBranch={checkoutBranch}
              callBack={refetchBranchs}
            />
          </div>
        </CollapsibleSection>

        <CollapsibleSection title="Remote Branch">
          {#each remoteBranchList as branch}
            <BranchItem
              type="remote"
              {branch}
              branchName={branch}
              bind:location
              bind:selectedBranch
              selectBranch={checkoutBranch}
              callBack={refetchBranchs}
            />
          {/each}
        </CollapsibleSection>
      </div>
      <div class="mt-auto">
        <Button buttonType="secondary" onClick={showOptionsDialog}>
          <div>
            <div>{userName}</div>
            <div>{userEmail}</div>
          </div>
        </Button>
      </div>

      <DialogBox bind:dialog={profileDialog}>
        <div class="flex flex-col m-8">
          <table class="border-collapse border">
            <thead>
              <tr>
                <th class="border">Name</th>
                <th class="border">Path</th>
              </tr>
            </thead>
            <tbody>
              {#each remotesList as remote}
                <tr>
                  <td class="border p-4">{remote["name"]}</td>
                  <td class="border p-4">{remote["url"]}</td>
                </tr>
              {/each}
            </tbody>
          </table>
          <Button buttonType="secondary" onClick={showNewRemoteDialog}
            >Add Remote</Button
          >
          <DialogBox bind:dialog={newRemoteDialog}>
            <div class="flex flex-col m-8">
              <label for="remoteName">Remote name:</label>
              <Input bind:value={newRemoteName} />
              <label for="remoteUrl">Remote url:</label>
              <Input bind:value={newRemoteURL} />
              <Button buttonType="secondary" onClick={addNewRemote}>Add</Button>
            </div>
          </DialogBox>
        </div>
      </DialogBox>
    </div>
    <div style="width: 80vw;">
      <div class="flex flex-row h-1/10">
        <Push
          location={location ?? ""}
          branchList={localBranchList}
          remoteList={remotesList}
          {selectedBranch}
        />

        <Pull
          location={location ?? ""}
          branchList={localBranchList}
          remoteList={remotesList}
          {selectedBranch}
        />
        <div class="p-2">
          {currentBranchRemoteCommitDelta[0]} to push
        </div>
        <div class="p-2">
          {currentBranchRemoteCommitDelta[1]} to pull
        </div>
      </div>
      <div class="flex h-1/10 tab-view-area">
        <TabSelect
          items={["graph", "commit"]}
          onSelected={tabChange}
          selected={mainTab}
        />
      </div>
      <div class="main-area h-8/10">
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
                    <div class="whitespace-nowrap">
                      {commitMessage[1]}
                    </div>
                    <div class="overflow-hidden text-ellipsis">
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
                <Input
                  placeholder="Type your message here."
                  type="textarea"
                  bind:value={makeCommitMessage}
                ></Input>
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

  .branch-area {
    width: 20vw;
  }

  .graph-area {
    display: flex;
    height: 99%;
    border-radius: 5px;
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

  .tab-view-area {
    background: var(--accent);
    height: 48px;
    border-radius: 4px;
  }

  .container {
    height: calc(100% - 30px);
    width: 100%;
    display: flex;
    padding: 0px;
    margin: 0px;
    max-width: none;
  }

  .main-area {
    width: -webkit-fill-available;
    height: calc(100% - 85px);
    background: var(--accent);
  }

  .commit-message {
    padding: 5px;
    width: -webkit-fill-available;
    text-align: start;
    background: transparent;
    color: var(----on-background-colorolor);
    border: none;
    border-radius: 4px;
  }

  .commit-message:hover {
    background: var(--hover);
  }

  .commit-message-info {
    display: flex;
    div {
      margin: 5px;
    }
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
    background: var(--hover);
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
