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
		class="border-border border-2 w-15 h-15 rounded {colour} cursor-pointer
			hover:border-text flex justify-center items-center
			peer-focus-visible:ring-2 peer-focus-visible:ring-blue-500"
		class:border-text={checked}>
		{#if checked}
			<Icon icon="tabler:check" class="text-text text-3xl" />
		{/if}
	</label>
</div>
