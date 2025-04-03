<script lang="ts">
    import Button from "$lib/ui-components/button.svelte";

    export let items: any[] = [];
    export let onClick: any;
    export let disable: boolean = false;

    let pos = { x: 0, y: 0 };
    let menu = { h: 0, w: 0 };
    let browser = { h: 0, w: 0 };
    let showMenu = false;

    function rightClickContextMenu(e: { clientX: any; clientY: any }) {
        showMenu = true;
        browser = {
            w: window.innerWidth,
            h: window.innerHeight,
        };
        pos = {
            x: e.clientX,
            y: e.clientY,
        };

        if (browser.h - pos.y < menu.h) pos.y = pos.y - menu.h;
        if (browser.w - pos.x < menu.w) pos.x = pos.x - menu.w;
    }

    function onPageClick(e: any) {
        showMenu = false;
    }

    function onBtnClick(e: any, itemId: string) {
        onClick(e, itemId);
    }
</script>

{#if disable}
    <slot />
{:else}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div on:contextmenu|preventDefault={rightClickContextMenu}>
        {#if showMenu}
            <div style="position: absolute; top:{pos.y}px; left:{pos.x}px">
                <div class="context-menu">
                    {#each items as item}
                        <Button
                            buttonType="secondary"
                            onClick={(e: any) => {
                                onBtnClick(e, item);
                            }}
                        >
                            {item}
                        </Button>
                    {/each}
                </div>
            </div>
        {/if}
        <slot />
    </div>
{/if}

<svelte:window on:click={onPageClick} />

<style>
    .context-menu {
        display: inline-flex;
        border: var(--border);
        width: 170px;
        background-color: var(--background-color);
        color: var(--on-background-color);
        border-radius: 3px;
        overflow: hidden;
        flex-direction: column;
    }
</style>
