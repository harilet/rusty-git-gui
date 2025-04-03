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
        <a
            class="pr-4 pl-4 tab-item"
            href="#"
            on:click={(e) => handleClick(e, item)}
        >
            <div class="mt-auto mb-auto">
                {item}
            </div>
            <span class="indicator {item === selected ? 'indicate' : ''}">
            </span>
        </a>
    {/each}
</div>

<style>
    .tab-group {
        border-radius: 5px;
    }

    .tab-item {
        display: flex;
        flex-direction: column;
        border-radius: 4px;
    }

    .tab-item:hover {
        background: var(--hover);
    }

    .indicator {
        margin-top: auto;
        background: transparent;
        height: 3px;
    }

    .indicate {
        background: var(--primary-color);
    }
</style>
