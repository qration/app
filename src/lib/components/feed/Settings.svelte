<script lang="ts">
	import { mode, setMode } from 'mode-watcher';
	import Modal from '../ui/Modal.svelte';
	import Button from '../ui/Button.svelte';
	import Icon from '@iconify/svelte';

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
				<label
					class="border-border border-2 w-15 h-15 rounded bg-light-theme
						cursor-pointer hover:border-text flex justify-center items-center"
					class:border-text={mode.current == 'light'}>
					{#if mode.current == 'light'}
						<Icon icon="tabler:check" class="text-text text-3xl" />
					{/if}
					<input
						type="radio"
						name="theme"
						id="light"
						checked={mode.current == 'light'}
						onclick={() => setMode('light')}
						class="opacity-0 fixed w-0" />
				</label>
				<label
					class="border-border border-2 w-15 h-15 rounded bg-dark-theme
						cursor-pointer hover:border-text flex justify-center items-center"
					class:border-text={mode.current == 'light'}>
					{#if mode.current == 'dark'}
						<Icon icon="tabler:check" class="text-text text-3xl" />
					{/if}
					<input
						type="radio"
						name="theme"
						id="light"
						checked={mode.current == 'dark'}
						onclick={() => setMode('dark')}
						class="opacity-0 fixed w-0" />
				</label>
			</div>
		</label>
		<div class="flex justify-end">
			<Button onclick={confirm}>Confirm</Button>
		</div>
	</div>
</Modal>
