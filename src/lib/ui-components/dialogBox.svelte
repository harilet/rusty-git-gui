<script lang="ts">
    import { onMount } from "svelte";

    export let dialog: HTMLDialogElement;

    onMount(() => {
        dialog.addEventListener("click", function (event) {
            var rect = dialog.getBoundingClientRect();
            var isInDialog =
                rect.top <= event.clientY &&
                event.clientY <= rect.top + rect.height &&
                rect.left <= event.clientX &&
                event.clientX <= rect.left + rect.width;
            if (!isInDialog) {
                dialog.close();
            }
        });
    });
</script>

<dialog bind:this={dialog} on:close>
    <slot />
</dialog>

<style>
    dialog {
        border: var(--border);
        background: var(--background-color);
        color: var(--on-background-color);
        border-radius: 5px;
    }
    dialog::backdrop {
        background: rgb(255 255 255 / 0.1);
    }
</style>
