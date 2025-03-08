<script>
    export let value = "";
    export let placeholder = "";
    export let type = "text";
    export let checked = false;
</script>

{#if type == "text"}
    <input type="text" {placeholder} bind:value />
{:else if type == "checkbox"}
    <input type="checkbox" {placeholder} {checked} />
{:else if type == "textarea"}
    <textarea {placeholder} bind:value></textarea>
{/if}

<style>
    input {
        background: var(--background);
        color: var(--on-background-color);
        border-radius: 3px;
        padding: 3px;
        border: solid 1px var(--semi-white);
        width: -webkit-fill-available;
    }

    textarea{
        background: var(--background);
        color: var(--on-background-color);
        border-radius: 3px;
        padding: 3px;
        border: solid 1px var(--semi-white);
    }
</style>
