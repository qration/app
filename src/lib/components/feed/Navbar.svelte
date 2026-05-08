<script lang="ts">
	import wordmark from '$lib/assets/qration-wordmark.svg';
	import TextInput from '$lib/components/ui/TextInput.svelte';
</script>

<div
	class="flex flex-row items-center gap-10 border-b-2 border-b-gray-300 bg-white
		p-4 text-4xl font-medium text-black">
	<div class="flex flex-1 justify-start">
		<img src={wordmark} alt="Qration logo" class="h-10 cursor-pointer" />
	</div>
	<TextInput placeholder="Search..." iconClass="ti ti-search" cls="flex-6" />
	<div class="flex flex-1 justify-end">
		<i class="ti ti-settings cursor-pointer"></i>
	</div>
</div>
