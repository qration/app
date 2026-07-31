<script lang="ts">
	import Icon from '@iconify/svelte';
	import { OverlayScrollbarsComponent } from 'overlayscrollbars-svelte';
	import { getOsbOptions } from '$lib/context/context.svelte';
	import { type TranscriptSnippet, type Transcript } from '$lib/util/bindings';

	let {
		transcript,
		ytPlayer,
	}: { transcript: Transcript | null; ytPlayer: HTMLIFrameElement } = $props();

	let open = $state(false);
	let loading = $state(false);
	let error = $state<string | null>(null);
	let lastSnippetTime: number | null = $derived(
		transcript?.snippets[transcript.snippets.length - 1].start || null,
	);

	function timestamp(seconds: number): string {
		const total = Math.floor(seconds);
		const h = Math.floor(total / 3600);
		const m = Math.floor((total % 3600) / 60);
		const s = total % 60;
		const mm =
			lastSnippetTime && lastSnippetTime > 60 * 10
				? String(m).padStart(2, '0')
				: String(m);
		const hh = lastSnippetTime && lastSnippetTime > 60 * 60 ? `${h}:` : '';
		return `${hh}${mm}:${String(s).padStart(2, '0')}`;
	}

	async function toggle() {
		open = !open;
	}

	async function onSnippetClick(snippet: TranscriptSnippet) {
		ytPlayer.contentWindow?.postMessage(
			JSON.stringify({
				event: 'command',
				func: 'seekTo',
				args: [snippet.start, true],
			}),
			'*',
		);
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
						{#if transcript}
							{#each transcript.snippets as snippet, i (i)}
								<button
									class="align-center flex cursor-pointer flex-row gap-3 rounded px-2 py-1 hover:bg-bg-hover"
									onclick={async () => await onSnippetClick(snippet)}>
									<span
										class="shrink-0 pt-0.75 font-mono text-sm text-text-muted tabular-nums">
										{timestamp(snippet.start ?? 0)}
									</span>
									<span class="min-w-0 text-left text-text"
										>{snippet.text}</span>
								</button>
							{/each}
						{/if}
					</div>
				</OverlayScrollbarsComponent>
			{/if}
		</div>
	{/if}
</div>
