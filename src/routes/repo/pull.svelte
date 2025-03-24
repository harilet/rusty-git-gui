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
        pullBranchName=selectedBranch;
        remoteName=remoteList[0]['name'];
    });

    let pullDialog: HTMLDialogElement;

    $: pullBranchName = "";
    $: remoteName = "";

    function showPullDialog() {
        pullDialog.showModal();
    }

    function pullBranch() {
        invoke("pull_from_remote", {
            repoLocation: location,
            branchName: pullBranchName,
            remote: remoteName,
        });
    }
</script>

<DialogBox bind:dialog={pullDialog}>
    <div class="flex flex-col m-8 w-40">
        <label for="remoteName">Branch</label>
        <select class="mb-4" bind:value={pullBranchName}>
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
        <Button buttonType="secondary" onClick={pullBranch}>Pull</Button>
    </div>
</DialogBox>

<div>
    <Button buttonType="secondary" onClick={showPullDialog}><div>Pull</div></Button>
</div>