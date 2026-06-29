<script lang="ts">
	import { openUrl } from '@tauri-apps/plugin-opener';
	import Button from '../ui/Button.svelte';
	import Modal from '../ui/Modal.svelte';

	let {
		open = $bindable(false),
		href,
	}: {
		open: boolean;
		href: string;
	} = $props();

	async function openLink() {
		await openUrl(href);
		open = false;
	}

	function cancelOpenLink() {
		open = false;
	}
</script>

<Modal bind:open title="Open External Link">
	<div class="flex flex-col gap-2">
		<div class="">This link is taking you to the following website:</div>
		<div
			class="font-mono px-4 py-2 rounded border-border border-2 bg-bg-secondary
				wrap-anywhere">
			{href}
		</div>
		<div class="flex flex-row gap-1 self-end">
			<Button onclick={cancelOpenLink} display="secondary">Close</Button>
			<Button onclick={openLink}>Visit Website</Button>
		</div>
	</div>
</Modal>
