<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
</script>

<div
	class="flex flex-row items-center gap-10 border-b-2 border-b-gray-300 bg-white
		p-4 text-4xl text-black">
	<div class="flex flex-1 justify-start">
	</div>
	<TextInput placeholder="Search..." iconClass="ti ti-search " cls="flex-5 " />
	<div class="flex flex-1 justify-end">
		<i class="ti ti-settings cursor-pointer"></i>
	</div>
</div>
