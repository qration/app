<script lang="ts">
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import ExternalLink from './dialogs/ExternalLink.svelte';

	import { openUrl } from '@tauri-apps/plugin-opener';
	import { feedStore } from '$lib/stores/feeds.svelte';

	import type { Article, Feed } from '$lib/util/bindings';
	import { onMount } from 'svelte';
	import ImageView from '../ui/ImageView.svelte';

	let {
		articleId,
		isOpen,
		onclose,
		ontransitionend,
	}: {
		articleId: string;
		isOpen: boolean;
		onclose: () => void;
		ontransitionend: () => void;
	} = $props();
	let article: Article | undefined = $derived(
		feedStore.data.articles.find((a) => a.id == articleId),
	);
	let feed: Feed | undefined = $derived(
		feedStore.data.feeds.find((f) => f.id == article?.feed_id),
	);

	let externalLinkOpen = $state(false);
	let imageViewOpen = $state(false);
	let suppressClick = $state(false);
	let href = $state('');
	let src = $state('');
	let alt = $state('');

	let articleViewRef: HTMLElement;

	function onDialogClose() {
		suppressClick = true;
		setTimeout(() => (suppressClick = false), 0);
	}

	function handleArticleClick(e: MouseEvent) {
		e.stopPropagation();
		e.preventDefault();
		if (suppressClick) return;
		if (handleLinkClick(e)) return;
		if (handleImageClick(e)) return;
	}

	function handleLinkClick(e: MouseEvent) {
		const a = (e.target as HTMLElement).closest('a');
		if (!a) return false;

		href = a.getAttribute('href') || '';
		href = href.replace(/\s+/g, '');
		if (!href) return false;
		externalLinkOpen = true;
		return true;
	}

	function handleImageClick(e: MouseEvent) {
		const img = (e.target as HTMLElement).closest('img');
		if (!img) return false;

		src = img.getAttribute('src') || '';
		src = src.replace(/\s+/g, '');
		if (!src) return false;
		alt = img.getAttribute('alt') || '';
		imageViewOpen = true;
		return true;
	}

	function handleTransitionEnd() {
		if (!isOpen) {
			ontransitionend();
		}
	}

	onMount(() => {
		const observer = new ResizeObserver(() => {
			if (!articleViewRef) return;
			articleViewRef.style.transition = 'none';
			void articleViewRef.offsetHeight;
			articleViewRef.style.transition = '';
		});

		observer.observe(document.body);

		return () => observer.disconnect();
	});
</script>

<div
	bind:this={articleViewRef}
	ontransitionend={handleTransitionEnd}
	class="fixed inset-y-0 z-50 flex h-full w-full min-w-0 flex-col transition-all duration-500 lg:static lg:translate-x-0 {isOpen
		? 'translate-x-0'
		: 'translate-x-full'} bg-bg pt-safe-top pb-safe-bottom"
	inert={externalLinkOpen || imageViewOpen}>
	{#if article && feed}
		<div
			class="flex max-w-full min-w-0 shrink-0 flex-row items-center
				justify-between border-b-2 border-border p-2 lg:justify-end">
			<IconButton
				icon="tabler:arrow-left"
				label="Back"
				onclick={() => onclose()}
				class="lg:hidden" />
			<div class="flex flex-row justify-end gap-1">
				<IconButton
					icon="tabler:bookmark{article.saved ? '-filled' : ''}"
					label="Save {article.name}"
					onclick={() => (article.saved = !article.saved)} />
				{#if article.url}
					<IconButton
						icon="tabler:external-link"
						label="Open {article.name}"
						onclick={async () => await openUrl(article.url!)} />
				{/if}
			</div>
		</div>
		<div class="flex h-full w-full flex-col gap-2 overflow-y-scroll p-5">
			<div
				class="flex max-w-full min-w-0 flex-col
					font-medium text-text">
				<span class="min-w-0 text-3xl font-bold">{article.name}</span>
				<span
					class="shrink-0 text-2xl font-light whitespace-nowrap text-text-secondary"
					>{feed.name}</span>
			</div>
			<!-- {#if article.media_type == 'video'}
				<iframe
					title={article.name}
					src={article.enclosure!.url}
					width="100%"
					height="allowfullscreen"
					frameborder="0"
					class="aspect-video"></iframe>
			{/if} -->
			<div
				onclick={handleArticleClick}
				role="presentation"
				class="feed-content h-min">
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				{@html article.content}
			</div>
		</div>
	{/if}
</div>

<ExternalLink bind:open={externalLinkOpen} {href} onclose={onDialogClose} />
<ImageView bind:open={imageViewOpen} {src} {alt} onclose={onDialogClose} />

<style>
	.feed-content :global(img) {
		display: block;
		margin: 0 auto;
		padding: 1rem 0rem;
		cursor: pointer;
	}

	@media screen and (min-width: 48rem) {
		.feed-content :global(img) {
			max-width: 80%;
		}
	}

	.feed-content > :global(*) {
		padding: 1rem 2rem;
	}

	.feed-content :global(a) {
		color: var(--color-text-link);
	}

	.feed-content :global(a:hover) {
		text-decoration: underline;
	}

	.feed-content :global(ul) {
		list-style: disc;
		padding-left: 2rem;
	}

	.feed-content :global(ul > li) {
		padding-left: 0.5rem;
	}
</style>
