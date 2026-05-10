<script lang="ts">
	import TextInput from '$lib/components/ui/TextInput.svelte';
</script>

<div
	class="flex flex-col justify-start h-full border-border border-r-2 py-3
		px-4">
	<div
		class="flex flex-row gap-10 border-border bg-bg text-4xl text-text">
		<TextInput placeholder="Search..." iconClass="ti ti-search " />
	</div>
</div>
