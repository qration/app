<script lang="ts">
	import IconButton from './IconButton.svelte';

	import type { Snippet } from 'svelte';

	let {
		open = $bindable(false),
		title = '',
		onclose,
		children,
	}: {
		open: boolean;
		title?: string;
		onclose?: () => void;
		children: Snippet;
	} = $props();

	let dialog: HTMLDialogElement | null = $state(null);

	function close() {
		open = false;
		dialog?.close();
		onclose?.();
	}

	$effect(() => {
		if (!open) dialog?.close();
		else dialog?.showModal();
	});
</script>

<dialog
	bind:this={dialog}
	class="mx-auto my-auto w-100 rounded-xl border
		border-border bg-bg p-6 text-text shadow-lg backdrop:bg-black/50
		backdrop:backdrop-blur-xs"
	aria-modal="true"
	aria-label={title || 'Dialog'}
	onclose={close}
	closedby="any">
	<div class="mb-4 flex items-center justify-between">
		<div class="text-xl font-medium">{title}</div>
		<IconButton icon="tabler:x" label="Close" onclick={close} />
	</div>
	{@render children()}
</dialog>

<style>
	dialog {
		opacity: 0;
		transform: scale(0.97);
		transition: all 0.1s ease-out allow-discrete;
	}

	dialog::backdrop {
		opacity: 0;
		background-color: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		transition: all 0.1s ease-out allow-discrete;
	}

	dialog[open] {
		opacity: 1;
		transform: scale(1);
	}

	dialog[open]::backdrop {
		opacity: 1;
	}

	@starting-style {
		dialog[open] {
			opacity: 0;
			transform: scale(0.97);
			&::backdrop {
				opacity: 0;
			}
		}
	}
</style>
