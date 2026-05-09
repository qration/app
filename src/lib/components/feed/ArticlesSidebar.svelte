<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
</script>

<div class="flex flex-col justify-start h-full border-gray-300 border-r-2 py-3 px-4">
	<div
		class="flex flex-row gap-10 border-gray-300 bg-white text-4xl text-black">
		<TextInput placeholder="Search..." iconClass="ti ti-search " />
	</div>
</div>
