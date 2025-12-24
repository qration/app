<script lang="ts">
	import Tag from "./Tag.svelte";


</script>

<div class="w-full border-2 border-gray-400 p-4 rounded-xl flex flex-col">
  <span class="text-3xl font-bold">Thing 1</span>
  <div class="flex flex-row gap-1">
    <Tag name="Tag 1"/>
    <Tag name="Tag 2"/>
  </div>
  <span>Body text</span>
</div>