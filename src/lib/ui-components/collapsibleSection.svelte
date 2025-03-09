<script lang="ts">
    import { onMount } from "svelte";

    export let title = "";
    export let defaultCollapsed = true;

    onMount(() => {
        collapsed = defaultCollapsed;
    });

    $: collapsed = true;

    function toggleSection() {
        collapsed = !collapsed;
    }
</script>

<!-- svelte-ignore a11y_invalid_attribute -->
<a href="#" on:click={toggleSection}>{title}</a>

{#if !collapsed}
    <slot />
{/if}
