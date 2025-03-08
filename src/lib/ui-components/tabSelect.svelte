<script lang="ts">
    export let items;
    export let onSelected: (item: any) => void;
    export let selected;

    export function handleClick(
        e: MouseEvent & { currentTarget: EventTarget & HTMLAnchorElement },
        item: any,
    ) {
        e.preventDefault();
        onSelected(item);
    }
</script>

<div class="flex tab-group">
    {#each items as item}
        <!-- svelte-ignore a11y_invalid_attribute -->
        <a class="pr-4 pl-4 tab-item {item===selected?'tab-item-selected':''}" href="#" on:click={(e) => handleClick(e, item)}>
            {item}
        </a>
    {/each}
</div>

<style>
    .tab-group {
        border: solid 1px #ccc;
        border-radius: 5px;
    }

    .tab-item:hover{
        background: #ffffff40;
    }

    .tab-item-selected{
        background: #ffffff40;
    }
</style>