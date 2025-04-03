<script lang="ts">
    import { fly } from "svelte/transition";

    export let message: string[] = [];

    $: visible = false;

    $: {
        console.log("toasty", message);
        if (message.length == 1 && message[1] == "") {
            message.pop();
            message = message;
        } else if (message.length > 0) {
            visible = true;

            setTimeout(() => {
                message.pop();
                message = message;
                if (message.length < 1) {
                    visible = false;
                }
            }, 5000);
        }
    }
</script>

{#if visible}
    <div class="toast">
        <div class="flex flex-col">
            {#each message as item}
                <div
                    class="toast-item"
                    in:fly={{ x: 200 }}
                    out:fly={{ x: 200 }}
                >
                    {item}
                </div>
            {/each}
        </div>
    </div>
{/if}

<style>
    .toast {
        position: fixed;
        bottom: 10px;
        right: 15px;
    }

    .toast-item {
        padding: 10px 20px;
        background-color: var(--primary-color);
        color: var(--background-color);
        border-radius: 4px;
        margin-top: 5px;
    }
</style>
