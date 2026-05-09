<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
</script>

<div class="flex flex-col justify-start h-full border-gray-300 border-r-2">
	<div
		class="flex flex-row gap-10 border-b-2 border-gray-300 bg-white text-4xl text-black">
		<TextInput placeholder="Search..." iconClass="ti ti-search " cls="flex-5 " />
	</div>
</div>
