<script lang="ts">
  let { text = '', iconClass = '' } = $props();
</script>

<button class="py-1 text-lg w-full h-min flex flex-row items-center gap-2 cursor-pointer">
  <i class={iconClass}></i>
  <div class="font-medium">
    {text}
  </div>
</button>