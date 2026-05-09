<script lang="ts">
  let { iconClass, label = '' } = $props();
</script>

<button class="py-1 px-2 text-2xl h-min cursor-pointer" aria-label={label}>
  <i class={iconClass}></i>
</button>