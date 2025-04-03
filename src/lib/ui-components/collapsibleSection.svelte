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
<a href="#" on:click={toggleSection} class="branch-name">{title}</a>

{#if !collapsed}
    <slot />
{/if}
<style>
    .branch-name {
  padding: 10px 5px;
  margin: 0px 5px;
  border-radius: 5px;
  cursor: pointer;
  border: none;
  width: -webkit-fill-available;
  background: none;
  color: var(--on-background-color);
  text-align: start;
  text-overflow: ellipsis;
  overflow: hidden;
  display: flex;
  align-items: center;
}
</style>