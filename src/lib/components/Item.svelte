<script lang="ts">
	import Tag from "./Tag.svelte";

  let { name, tags, body }: {
    name: string,
    tags: Array<string>,
    body: string,
  } = $props();
</script>

<div class="w-full border-2 border-gray-400 p-4 rounded-xl flex flex-col">
  <span class="text-3xl font-bold">{name}</span>
  <div class="flex flex-row gap-1">
    {#each tags as tag}
      <Tag name={tag}/>
    {/each}
  </div>
  <span>{body}</span>
</div>