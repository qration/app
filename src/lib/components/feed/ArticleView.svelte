<script lang="ts">
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import ExternalLink from './dialogs/ExternalLink.svelte';

	import { openUrl } from '@tauri-apps/plugin-opener';
	import { getFeedStore, getMQ } from '$lib/context/context.svelte';

	import {
		commands,
		type ArticleContent,
		type ArticleLight,
		type Feed,
	} from '$lib/util/bindings';
	import { onMount } from 'svelte';
	import ImageView from '../ui/ImageView.svelte';

	let {
		articleId,
		articleContainer = $bindable(),
		isOpen,
		onclose,
		ontransitionend,
	}: {
		articleId: string;
		articleContainer: HTMLDivElement | null;
		isOpen: boolean;
		onclose: () => void;
		ontransitionend: () => void;
	} = $props();

	const feedStore = getFeedStore();
	const MQ = getMQ();

	let article: ArticleLight | undefined = $derived(
		feedStore.articles_light.find((a) => a.id == articleId),
	);
	let feed: Feed | undefined = $derived(
		feedStore.feeds.find((f) => f.id == article?.feed_id),
	);

	let articleContent: ArticleContent | null = $state(null);

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

	function handleArticleKeydown(e: KeyboardEvent) {
		if (e.key != 'Enter') return;
		e.stopPropagation();
		e.preventDefault();
		if (handleImageKeydown(e)) return;
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
		return openImage(img);
	}

	function handleImageKeydown(e: KeyboardEvent) {
		const img = (e.target as HTMLElement).closest('img');
		if (!img) return false;
		return openImage(img);
	}

	function openImage(img: HTMLImageElement) {
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

	$effect(() => {
		commands.fetchArticleContent(articleId).then((res) => {
			console.log(res);
			if (res.status == 'ok') articleContent = res.data;
		});

		if (!articleContainer) return;
		for (const img of articleContainer.querySelectorAll('img')) {
			img.tabIndex = 0;
			img.setAttribute('role', 'button');
			img.setAttribute(
				'aria-label',
				`View image${img.alt ? `: ${img.alt}` : ''}`,
			);
		}
	});
</script>

<div
	bind:this={articleViewRef}
	ontransitionend={handleTransitionEnd}
	class="fixed inset-y-0 z-50 flex h-full w-full min-w-0 flex-col transition-all duration-500 lg:static lg:translate-x-0 {isOpen
		? 'translate-x-0'
		: 'translate-x-full'} bg-bg pt-safe-top pb-safe-bottom"
	inert={!MQ.current && !isOpen}>
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
					icon="tabler:bookmark{article.article_saved ? '-filled' : ''}"
					label="Save {article.article_name}"
					onclick={async () => {
						console.log(article.article_saved);
						await commands.setSaveArticle(articleId, !article.article_saved);
						article.article_saved = !article.article_saved;
					}} />
				{#if article.article_url}
					<IconButton
						icon="tabler:external-link"
						label="Open {article.article_name}"
						onclick={async () => await openUrl(article.article_url!)} />
				{/if}
			</div>
		</div>
		<div
			class="flex h-full w-full flex-col gap-2 overflow-y-scroll p-5"
			tabindex="-1"
			bind:this={articleContainer}>
			<div
				class="flex max-w-full min-w-0 flex-col
					font-medium text-text">
				<span class="min-w-0 text-3xl font-bold">{article.article_name}</span>
				<span
					class="shrink-0 text-2xl font-light whitespace-nowrap text-text-secondary"
					>{feed.feed_name}</span>
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
				onkeydown={handleArticleKeydown}
				role="presentation"
				class="feed-content h-min">
				<!-- eslint-disable-next-line svelte/no-at-html-tags -->
				{@html articleContent?.content}
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
