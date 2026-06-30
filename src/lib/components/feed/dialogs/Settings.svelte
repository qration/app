<script lang="ts">
	import Modal from '$lib/components/ui/Modal.svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Swatch from '$lib/components/ui/Swatch.svelte';

	import { mode, setMode } from 'mode-watcher';

	let {
		open = $bindable(false),
		onconfirm,
	}: {
		open: boolean;
		onconfirm?: () => void;
	} = $props();

	function confirm() {
		onconfirm?.();
		open = false;
	}
</script>

<Modal bind:open title="Settings">
	<div class="flex flex-col gap-4">
		<label class="flex flex-col gap-2">
			<span class="text-text-muted">Theme</span>
			<div class="flex flex-row gap-4">
				<Swatch
					colour="bg-light-theme"
					name="theme"
					id="light"
					onclick={() => setMode('light')}
					checked={mode.current == 'light'} />
				<Swatch
					colour="bg-dark-theme"
					name="theme"
					id="dark"
					onclick={() => setMode('dark')}
					checked={mode.current == 'dark'} />
			</div>
		</label>
		<div class="flex justify-end">
			<Button onclick={confirm}>Confirm</Button>
		</div>
	</div>
</Modal>
