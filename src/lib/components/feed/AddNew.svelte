<script lang="ts">
	import type { AddNewFormFields } from '$lib/util/interfaces';
	import Button from '../ui/Button.svelte';
	import Modal from '../ui/Modal.svelte';
	import Icon from '@iconify/svelte';
	import TextInput from '../ui/TextInput.svelte';
	import { isValidURL } from '$lib/util/util';

	let {
		open = $bindable(false),
		onconfirm,
	}: {
		open: boolean;
		onconfirm?: (url: string) => void;
	} = $props();

	const initialFormData: AddNewFormFields = {
		url: '',
	};

	const initialErrorData: Record<keyof AddNewFormFields, string> = {
		url: '',
	};

	let formData = $state(initialFormData);
	let errorData = $state(initialErrorData);

	function confirm() {
		const trimmed = formData.url.trim();
		if (!trimmed) {
			errorData.url = "URL can't be empty!";
			return;
		}

		if (!isValidURL(trimmed)) {
			errorData.url = 'URL must be valid!';
			return;
		}

		onconfirm?.(trimmed);
		formData.url = '';
		open = false;
	}

	function clearError(field: keyof AddNewFormFields) {
		errorData[field] = '';
	}

	function close() {
		formData = initialFormData;
		errorData = initialErrorData;
	}
</script>

<Modal bind:open onclose={close} title="Add New Feed">
	<div class="flex flex-col gap-4">
		<label class="flex flex-col gap-2">
			<span class="text-text-muted">Feed URL</span>
			<TextInput
				bind:input={formData.url}
				placeholder="https://example.com/feed.xml"
				clear={false}
				error={errorData.url != ''}
				oninput={() => clearError('url')} />
			{#if errorData.url}
				<span class="text-error flex flex-row items-center gap-1">
					<Icon icon="tabler:circle-x" />
					{errorData.url}
				</span>
			{/if}
		</label>
		<div class="flex justify-end">
			<Button onclick={confirm}>Confirm</Button>
		</div>
	</div>
</Modal>
