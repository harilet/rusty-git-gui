<script lang="ts">
    export let buttonType = "primary";
    export let onClick;
    export let disabled = false;
    export let isSelected = false;
</script>

{#if buttonType === "primary"}
    <button
        on:click={onClick}
        class="btn bth-primary {disabled ? 'btn-disabled' : ''}"
        {disabled}
    >
        <slot />
    </button>
{:else if buttonType === "secondary"}
    <button on:click={onClick} class="btn bth-secondary" {disabled}>
        <slot />
    </button>
{:else if buttonType === "select"}
    <button
        on:click={onClick}
        class="btn bth-select"
        {disabled}
        style={isSelected ? "background: var(--semi-white)" : "transparent"}
    >
        <slot />
    </button>
{:else}
    <p>Button Type not found</p>
{/if}

<style>
    .btn {
        padding: 5px;
        border-radius: 5px;
        width: var(--width);
    }

    .bth-primary {
        background: var(--primary-color);
        color: var(--on-primary-color);
    }

    .bth-secondary {
        background: var(--background-color);
        color: var(--on-background-color);
        border: solid 1px var(--semi-white);
    }

    .bth-select {
        color: var(--on-background-color);
    }

    .btn-disabled {
        background: rgb(255 255 255 / 60%) !important;
    }
</style>
