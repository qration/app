<script lang="ts">
	import Logo from '$lib/assets/favicon.svg?component';
	import Wordmark from '$lib/assets/qration-wordmark.svg?component';
	import { resolve } from '$app/paths';
	import Button from '$lib/components/ui/Button.svelte';
</script>

<div
	class="p-safe flex h-full w-full flex-col items-center justify-center gap-4">
	<div class="flex flex-row items-center gap-4">
		<Logo height="75" />
		<Wordmark height="50" class="fill-text" />
	</div>

	<a href={resolve('/feed')} data-sveltekit-replacestate>
		<Button onclick={() => {}}>Go to the app</Button>
	</a>
</div>
