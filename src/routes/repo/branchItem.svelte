<script lang="ts">
  export let branch;
  export let branchName;
  export let location: string | null;
  export let selectedBranch;
  export let callBack: () => void;
  export let selectBranch: (branch: string) => void;
  export let type = "local";

  import ContextMenu from "$lib/ui-components/contextMenu.svelte";
  import DialogBox from "$lib/ui-components/dialogBox.svelte";
  import Input from "$lib/ui-components/input.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Button from "$lib/ui-components/button.svelte";

  $: force = false;
  $: newBranchName = "";

  let renameBranchDialog: HTMLDialogElement;
  let deleteBranchDialog: HTMLDialogElement;

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

  function renameBranch(branch: string) {
    invoke("rename_branch", {
      repoLocation: location,
      branchName: branch,
      newBranchName: newBranchName,
      force: force,
    }).then(function () {
      callBack();
    });
  }

  function deleteBranch(branch: string) {
    invoke("delete_branch", {
      repoLocation: location,
      branchName: branch,
    }).then(function () {
      callBack();
    });
  }
</script>

<ContextMenu
  disable={type != "local"}
  items={["rename", "delete"]}
  onClick={branchContextMenuHandler}
>
  <button
    on:dblclick={() => selectBranch(branch)}
    class="branch-name {selectedBranch == branch ? 'branch-selected' : ''}"
  >
    {branchName}
  </button>
</ContextMenu>

<DialogBox bind:dialog={renameBranchDialog}>
  <div class="flex flex-col m-8">
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">Rename</label>
      <Input bind:value={newBranchName} placeholder="Rename" />
    </div>
    <div class="flex w-full max-w-sm flex-col gap-1.5">
      <label for="email">Force</label>
      <Input type="checkbox" bind:checked={force} />
    </div>
    <Button buttonType="secondary" onClick={(_: any) => renameBranch(branch)}
      >Rename</Button
    >
  </div>
</DialogBox>

<DialogBox bind:dialog={deleteBranchDialog}>
  <div class="flex flex-col m-8">
    <Button buttonType="secondary" onClick={(_: any) => deleteBranch(branch)}
      >Delete {branch}</Button
    >
  </div>
</DialogBox>

<style>
  .branch-name {
    padding: 10px 5px;
    margin: 0px 5px;
    border-radius: 5px;
    cursor: pointer;
    border: none;
    width: -webkit-fill-available;
    background: none;
    color: var(--on-background-color);
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
</style>
