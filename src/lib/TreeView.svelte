<script lang="ts">
  export let tree;

  function getLabel(data: string) {
    console.log(data);
    return data.split("/")[0];
  }

  function getChildren(data: string) { 
    return data.split("/")[0];;
  }

  function checkChild(data: string) {
    return data.indexOf("/") > 0;
  }
</script>

<div>
  {#each tree as item}
    <div>
      <div>
        {getLabel(item)}
      </div>
      {#if checkChild(item)}
        <svelte:self tree={getChildren(item)} />
      {/if}
    </div>
  {/each}
</div>

<style>

</style>
