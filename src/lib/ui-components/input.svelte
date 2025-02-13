<script>
    export let value = "";
    export let placeholder = "";
</script>

<input type="text" {placeholder} bind:value />

<style>
    input {
        background: var(--primary-color);
        color: var(--on-primary-color);
        border-radius: 3px;
        padding: 3px;
    }
</style>
