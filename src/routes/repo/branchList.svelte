<script lang="ts">
    import CollapsibleSection from "$lib/ui-components/collapsibleSection.svelte";
    import { onMount } from "svelte";
    import BranchItem from "./branchItem.svelte";

    export let branchList: string[];
    export let location: string | null;
    export let selectedBranch: string;
    export let callBack: () => void;
    export let selectBranch: (branch: string) => void;

    let branchObj: any = {};

    onMount(() => {
        branchList = branchList.sort(function (a: any, b: any) {
            return a.toLowerCase().localeCompare(b.toLowerCase());
        });
        branchList.forEach((branch: string) => {
            let branchNameItems = branch.split("/");

            if (branchNameItems.length > 1) {
                let prev = "";
                branchNameItems.forEach((item: string) => {
                    if (prev != "") {
                        let temp: string[] = branchObj[prev];
                        if (temp != undefined) {
                            temp.push(item);
                            branchObj[prev] = temp;
                        } else {
                            branchObj[prev] = [item];
                        }
                    }
                    prev = item;
                });
            } else {
                branchObj[branch] = null;
            }
        });
    });
</script>

{#each Object.entries(branchObj) as key, _}
    {#if key[1] == null}
        <BranchItem
            branch={key[0]}
            branchName={key[0]}
            bind:location
            bind:selectedBranch
            {selectBranch}
            {callBack}
        />
    {:else}
        <CollapsibleSection title={key[0]} defaultCollapsed={true}>
            <div class="ml-3">
                {#each Object.entries(key[1]) as subBranch, _}
                    <BranchItem
                        branch={key[0] + "/" + subBranch[1]}
                        branchName={subBranch[1]}
                        bind:location
                        bind:selectedBranch
                        {selectBranch}
                        {callBack}
                    />
                {/each}
            </div>
        </CollapsibleSection>
    {/if}
{/each}
