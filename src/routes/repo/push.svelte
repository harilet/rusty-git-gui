<script lang="ts">
    import DialogBox from "$lib/ui-components/dialogBox.svelte";
    import Button from "$lib/ui-components/button.svelte";
    import Input from "$lib/ui-components/input.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    export let location = "";
    export let branchList;
    export let remoteList;
    export let selectedBranch;

    onMount(() => {
        pushBranchName=selectedBranch;
        remoteName=remoteList[0]['name'];
    });


    let pushDialog: HTMLDialogElement;

    $: pushBranchName = "";
    $: remoteName = "";

    function showPushDialog() {
        pushDialog.showModal();
    }

    function pushBranch() {
        invoke("push_to_remote", {
            repoLocation: location,
            branchName: pushBranchName,
            remote: remoteName,
        });
    }
</script>

<DialogBox bind:dialog={pushDialog}>
    <div class="flex flex-col m-8 w-40">
        <label for="remoteName">Branch</label>
        <select class="mb-4" bind:value={pushBranchName}>
            {#each branchList as branch}
              <option value={branch} selected>{branch}</option>
            {/each}
          </select>
        <label for="remoteUrl">Remote</label>
        <select class="mb-4" bind:value={remoteName}>
            {#each remoteList as remote}
              <option value={remote['name']} selected>{remote['name']}</option>
            {/each}
          </select>
        <Button buttonType="secondary" onClick={pushBranch}>Push</Button>
    </div>
</DialogBox>

<div>
    <Button buttonType="secondary" onClick={showPushDialog}><div>Push</div></Button>
</div>
