<script lang="ts">
	import IconButton from './IconButton.svelte';

	let {
		open = $bindable(false),
		alt = '',
		src = '',
		onclose,
	}: {
		open: boolean;
		alt?: string;
		src?: string;
		onclose?: () => void;
	} = $props();

	let dialog: HTMLDialogElement | null = $state(null);

	function close() {
		open = false;
		dialog?.close();
		onclose?.();
	}

	function handleClick(e: MouseEvent) {
		if (!(e.target as HTMLElement).closest('img')) close();
	}

	$effect(() => {
		console.log('what');
		if (!open) dialog?.close();
		else dialog?.showModal();
	});
</script>

<dialog
	bind:this={dialog}
	class="m-0 h-dvh max-h-none w-screen max-w-none bg-transparent p-0 text-text
    backdrop:bg-black/50 backdrop:backdrop-blur-xs"
	aria-modal="true"
	aria-label={alt || 'Image'}
	onclose={close}
	onclick={handleClick}
	closedby="any">
	<IconButton
		class="fixed top-2 left-4 z-50 pt-safe-top pl-safe-left"
		icon="tabler:x"
		label="Close"
		onclick={close} />
	<div class="flex h-full w-full items-center justify-center p-4">
		<img {src} {alt} class="max-h-full" />
	</div>
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
