<script lang="ts">
	import type { Snippet } from 'svelte';
	import IconButton from './IconButton.svelte';

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

	function onDialogClick(e: MouseEvent) {
		if (e.target === dialog) close();
	}

	$effect(() => {
		console.log('what');
		if (!open) dialog?.close();
		else dialog?.showModal();
	});
</script>

<dialog
	bind:this={dialog}
	class="static mx-auto my-auto bg-bg text-text border-2 border-border
		rounded-xl p-6 min-w-96 max-w-[90vw] shadow-lg backdrop:bg-black/50
		backdrop:backdrop-blur-xs"
	aria-modal="true"
	aria-label={title || 'Dialog'}
	onclose={close}
	onclick={onDialogClick}>
	<div class="flex justify-between items-center mb-4">
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
