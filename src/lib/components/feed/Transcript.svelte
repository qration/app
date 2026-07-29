<script lang="ts">
	import Icon from '@iconify/svelte';
	import { OverlayScrollbarsComponent } from 'overlayscrollbars-svelte';
	import { getOsbOptions } from '$lib/context/context.svelte';
	import { commands, type TranscriptSnippet } from '$lib/util/bindings';

	let { videoId }: { videoId: string } = $props();

	let open = $state(false);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let snippets: TranscriptSnippet[] = $state([]);

	function timestamp(seconds: number): string {
		const total = Math.floor(seconds);
		const h = Math.floor(total / 3600);
		const m = Math.floor((total % 3600) / 60);
		const s = total % 60;
		const mm = h > 0 ? String(m).padStart(2, '0') : String(m);
		return `${h > 0 ? `${h}:` : ''}${mm}:${String(s).padStart(2, '0')}`;
	}

	async function toggle() {
		open = !open;
		if (!open || loading || snippets.length > 0) return;

		loading = true;
		error = null;
		const res = await commands.fetchTranscript(videoId);
		if (res.status == 'ok') {
			snippets = res.data;
			if (snippets.length == 0) error = 'No transcript available.';
		} else {
			error =
				res.error == 'TranscriptUnavailable'
					? 'No transcript available for this video.'
					: "Couldn't load the transcript.";
		}
		loading = false;
	}
</script>

<div class="w-full rounded-lg bg-bg-secondary">
	<button
		type="button"
		onclick={toggle}
		aria-expanded={open}
		class="flex w-full cursor-pointer items-center justify-between gap-2
			rounded-lg p-3 text-left select-none hover:bg-bg-hover">
		<span class="font-medium text-text">Transcript</span>
		<Icon
			icon="tabler:chevron-down"
			class="shrink-0 text-xl text-text-muted transition-transform duration-200 {open
				? 'rotate-180'
				: ''}" />
	</button>

	{#if open}
		<div class="border-t border-border">
			{#if loading}
				<p class="p-3 text-sm text-text-secondary">Loading transcript…</p>
			{:else if error}
				<p class="p-3 text-sm text-text-secondary">{error}</p>
			{:else}
				<OverlayScrollbarsComponent
					defer
					class="max-h-96 w-full"
					options={getOsbOptions()}>
					<div class="flex flex-col gap-0.5 p-2">
						{#each snippets as snippet, i (i)}
							<div class="flex flex-row gap-3 rounded px-2 py-1">
								<span
									class="shrink-0 pt-0.5 text-sm text-text-muted tabular-nums">
									{timestamp(snippet.start ?? 0)}
								</span>
								<span class="min-w-0 text-text">{snippet.text}</span>
							</div>
						{/each}
					</div>
				</OverlayScrollbarsComponent>
			{/if}
		</div>
	{/if}
</div>
