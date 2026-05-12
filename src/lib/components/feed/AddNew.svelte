<script lang="ts">
	import Button from '../ui/Button.svelte';
	import Modal from '../ui/Modal.svelte';

	let {
		open = $bindable(false),
		onconfirm,
	}: {
		open: boolean;
		onconfirm?: (url: string) => void;
	} = $props();

	let url = $state('');

	function confirm() {
		const trimmed = url.trim();
		if (!trimmed) return;
		onconfirm?.(trimmed);
		url = '';
		open = false;
	}
</script>

<Modal bind:open title="Add New Feed">
	<div class="flex flex-col gap-4">
		<label class="flex flex-col gap-2">
			<span class="text-text-muted">Feed URL</span>
			<input
				type="url"
				placeholder="https://example.com/feed.xml"
				class="w-full rounded border-2 border-border bg-transparent px-3 py-2
					text-text placeholder:text-text-secondary focus:outline-none focus:ring-0 focus:border-text"
				bind:value={url} />
		</label>
		<div class="flex justify-end">
			<Button onclick={confirm}>Confirm</Button>
		</div>
	</div>
</Modal>
