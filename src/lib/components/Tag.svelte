<script lang="ts">
  let { name }: { name: string } = $props();
</script>

<span class="bg-blue-100 text-blue-600
             px-2 py-1 my-1
             rounded-xl border-2
             cursor-pointer hover:bg-blue-200">
  {name}
</span>