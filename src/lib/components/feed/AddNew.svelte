<script lang="ts">
	import type { AddNewFormFields } from '$lib/util/interfaces';
	import Button from '../ui/Button.svelte';
	import Modal from '../ui/Modal.svelte';
	import Icon from '@iconify/svelte';
	import TextInput from '../ui/TextInput.svelte';
	import { isValidURL } from '$lib/util/util';
	import { invoke } from '@tauri-apps/api/core';
	import { feedStore } from '$lib/stores/feeds.svelte';
	import type { FeedWithArticles } from '$lib/util/bindings';

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

		console.log('hi');

		const ch = await invoke<FeedWithArticles>('new_feed', {
			url: formData.url,
		}).catch((err: string) => {
			errorData.url = `Error: ${err}`;
		});

		console.log(ch);

		if (errorData.url || !ch) return;
		feedStore.data.feeds.push(ch.feed);
		feedStore.data.articles = feedStore.data.articles.concat(ch.articles);

		console.log(feedStore);

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
	<form>
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
				<Button
					onclick={async () => {
						loading = true;
						await confirm();
						loading = false;
					}}
					class="h-10"
					type="submit">
					<span class="flex justify-center items-center w-15">
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
