<script lang="ts">
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';

	import { openUrl } from '@tauri-apps/plugin-opener';

	let {
		open = $bindable(false),
		href,
		onclose,
	}: {
		open: boolean;
		href: string;
		onclose: () => void;
	} = $props();

	async function openLink() {
		await openUrl(href);
		open = false;
	}

	function cancelOpenLink() {
		open = false;
	}
</script>

<Modal bind:open title="Open External Link" {onclose}>
	<div class="flex flex-col gap-2">
		<div>This link is taking you to the following website:</div>
		<div
			class="rounded border border-border bg-bg-secondary px-4 py-2 font-mono
				wrap-anywhere">
			{href}
		</div>
		<div class="flex flex-row gap-1 self-end">
			<Button onclick={cancelOpenLink} display="secondary">Close</Button>
			<Button onclick={openLink}>Visit Website</Button>
		</div>
	</div>
</Modal>
