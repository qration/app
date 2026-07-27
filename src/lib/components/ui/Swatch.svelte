<script lang="ts">
	import Icon from '@iconify/svelte';

	let {
		checked,
		onclick,
		id,
		name,
		colour,
	}: {
		checked: boolean;
		onclick: () => void;
		id: string;
		name: string;
		colour: string;
	} = $props();
</script>

<div>
	<input type="radio" {name} {id} {checked} {onclick} class="peer sr-only" />
	<label
		for={id}
		class="h-15 w-15 rounded border border-border {colour} flex
			cursor-pointer items-center justify-center peer-focus-visible:ring-2
			peer-focus-visible:ring-blue-500"
		class:border-text={checked}>
		{#if checked}
			<Icon icon="tabler:check" class="text-3xl text-text" />
		{/if}
	</label>
</div>
