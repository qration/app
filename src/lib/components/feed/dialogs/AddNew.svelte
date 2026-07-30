<script lang="ts">
	import Icon from '@iconify/svelte';
	import Button from '$lib/components/ui/Button.svelte';
	import Modal from '$lib/components/ui/Modal.svelte';
	import TextInput from '$lib/components/ui/TextInput.svelte';

	import { isValidURL } from '$lib/util/util';
	import { getFeedStore } from '$lib/context/context.svelte';
	const feedStore = getFeedStore();

	import type { AddNewFormFields } from '$lib/util/interfaces';
	import { commands } from '$lib/util/bindings';

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
	let loading = $state(false);

	async function confirm() {
		const trimmed = formData.url.trim();

		if (!trimmed) {
			errorData.url = "URL can't be empty!";
			return;
		}

		if (!isValidURL(trimmed)) {
			errorData.url = 'URL must be valid!';
			return;
		}

		const res = await commands.newFeed(formData.url);
		if (res.status == 'error') {
			errorData.url = `Error: ${res.error}`;
			return;
		}

		const ch = res.data;

		feedStore.feeds.push(ch.feed);
		feedStore.articles_light = feedStore.articles_light.concat(
			ch.articles_light,
		);

		onconfirm?.(trimmed);
		formData.url = '';
		open = false;
	}

	// Without preventDefault the native submit navigates, which resets the
	// history state the sidebar's collapsed flag lives in.
	async function submit(e: SubmitEvent) {
		e.preventDefault();
		if (loading) return;
		loading = true;
		await confirm();
		loading = false;
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
	<form onsubmit={submit}>
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
					<span class="flex flex-row items-center gap-1 text-error">
						<Icon icon="tabler:circle-x" />
						{errorData.url}
					</span>
				{/if}
			</label>
			<div class="flex justify-end">
				<Button class="h-10" type="submit">
					<span class="flex w-15 items-center justify-center">
						{#if loading}
							<Icon icon="tabler:loader-2" class="block animate-spin" />
						{:else}
							Confirm
						{/if}
					</span>
				</Button>
			</div>
		</div>
	</form>
</Modal>
