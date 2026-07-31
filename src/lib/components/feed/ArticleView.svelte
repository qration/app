<script lang="ts">
	import IconButton from '$lib/components/ui/IconButton.svelte';
	import ExternalLink from './dialogs/ExternalLink.svelte';
	import Transcript from './Transcript.svelte';

	import { openUrl } from '@tauri-apps/plugin-opener';
	import {
		getFeedStore,
		getMQ,
		getOsbOptions,
	} from '$lib/context/context.svelte';

	import {
		commands,
		type ArticleContent,
		type Transcript as ArticleTranscript,
		type ArticleLight,
		type Feed,
	} from '$lib/util/bindings';
	import { onMount } from 'svelte';
	import ImageView from '../ui/ImageView.svelte';
	import { countWords, readingMinutes, youtubeVideoId } from '$lib/util/util';
	import Icon from '@iconify/svelte';
	import { OverlayScrollbarsComponent } from 'overlayscrollbars-svelte';

	let {
		articleId,
		articleContainer = $bindable(),
		isOpen,
		zen,
		onclose,
		onzentoggle,
		ontransitionend,
	}: {
		articleId: string;
		articleContainer: HTMLDivElement | null;
		isOpen: boolean;
		zen: boolean;
		onclose: () => void;
		onzentoggle: () => void;
		ontransitionend: () => void;
	} = $props();

	const feedStore = getFeedStore();
	const MQ = getMQ();
	const dtf = new Intl.DateTimeFormat('en-CA', {
		dateStyle: 'long',
		timeStyle: 'short',
	});

	let article: ArticleLight | undefined = $derived(
		feedStore.articles_light.find((a) => a.id == articleId),
	);
	let feed: Feed | undefined = $derived(
		feedStore.feeds.find((f) => f.id == article?.feed_id),
	);
	let videoId: string | null = $derived(
		feed?.feed_type == 'youtube'
			? youtubeVideoId(article?.article_url ?? null)
			: null,
	);

	let articleContent: ArticleContent | null = $state(null);
	let transcript: ArticleTranscript | null = $state(null);
	let minutes: number | null = $derived.by(() =>
		readingMinutes(countWords(articleContent?.content)),
	);

	let externalLinkOpen = $state(false);
	let imageViewOpen = $state(false);
	let suppressClick = $state(false);
	let href = $state('');
	let src = $state('');
	let alt = $state('');

	let articleViewRef: HTMLElement;
	let ytPlayer: HTMLIFrameElement | null = $state(null);

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
		if (handleLinkClick(e)) return;
		if (handleImageClick(e)) return;
	}

	function handleLinkClick(e: Event) {
		const a = (e.target as HTMLElement).closest('a');
		if (!a) return false;

		href = a.getAttribute('href') || '';
		href = href.replace(/\s+/g, '');
		if (!href) return false;
		externalLinkOpen = true;
		return true;
	}

	function handleImageClick(e: Event) {
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
		if (!articleId) return;
		commands.fetchArticleContent(articleId).then((res) => {
			if (res.status == 'ok') [articleContent, transcript] = [...res.data];
			console.log(res);
		});
	});

	$effect(() => {
		void articleContent?.content;
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
			class="m-4 flex max-w-full min-w-0 shrink-0 flex-row
				items-center justify-between rounded-lg bg-bg-secondary p-2 lg:justify-end">
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
						await commands.setSaveArticle(articleId, !article.article_saved);
						article.article_saved = !article.article_saved;
					}} />
				{#if article.article_url}
					<IconButton
						icon="tabler:external-link"
						label="Open {article.article_name}"
						onclick={async () => await openUrl(article.article_url!)} />
				{/if}
				{#if MQ.current}
					<IconButton
						icon="tabler:arrows-diagonal{zen ? '-minimize-2' : ''}"
						label={zen ? 'Exit zen mode' : 'Zen mode'}
						onclick={() => onzentoggle()} />
				{/if}
			</div>
		</div>
		{#key articleId}
			<OverlayScrollbarsComponent
				defer
				class="h-full w-full"
				options={getOsbOptions()}>
				<div
					role="tabpanel"
					tabindex={0}
					bind:this={articleContainer}
					class="mx-auto flex w-full flex-col gap-5 px-5 pb-5 transition-[max-width] duration-500 {zen
						? 'max-w-3xl'
						: 'max-w-full'}">
					<div class="flex max-w-full min-w-0 flex-col font-medium text-text">
						<span
							class="shrink-0 text-2xl font-light whitespace-nowrap text-text"
							>{feed.feed_name}</span>
						<span class="min-w-0 text-3xl font-bold"
							>{article.article_name}</span>
						<span class="shrink-0 text-lg font-light text-text-secondary"
							>{dtf.format(article.article_date * 1000)}</span>
					</div>
					<hr class="text-border" />
					{#if videoId}
						<iframe
							bind:this={ytPlayer}
							title={article.article_name}
							src="https://www.youtube-nocookie.com/embed/{videoId}?enablejsapi=1"
							allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
							allowfullscreen
							class="aspect-video w-full rounded-lg border-0"></iframe>
						<Transcript {transcript} {ytPlayer} />
					{/if}
					<div
						onclick={handleArticleClick}
						onkeydown={handleArticleKeydown}
						role="presentation"
						class="feed-content flex h-full flex-col gap-3">
						<!-- eslint-disable-next-line svelte/no-at-html-tags -->
						{@html articleContent?.content}
					</div>
				</div>
			</OverlayScrollbarsComponent>
		{/key}
		{#if minutes && !videoId && MQ.current}
			<div
				aria-hidden={!zen}
				class="pointer-events-none absolute bottom-0 left-0 m-4 flex
					flex-row items-center gap-1.5 rounded-full border border-border
					bg-bg-secondary px-3 py-1.5 text-sm font-light text-text-secondary
					transition-all duration-500 select-none {zen
					? 'translate-y-0 opacity-60'
					: 'translate-y-2 opacity-0'}">
				<Icon icon="tabler:clock" class="shrink-0 text-base" />
				<span class="tabular-nums">{minutes} min read</span>
			</div>
		{/if}
	{/if}
</div>

<ExternalLink bind:open={externalLinkOpen} {href} onclose={onDialogClose} />
<ImageView bind:open={imageViewOpen} {src} {alt} onclose={onDialogClose} />

<style>
	.feed-content :global(img) {
		display: block;
		margin: 0 auto;
		margin: 1rem auto;
		cursor: pointer;
	}

	@media screen and (min-width: 48rem) {
		.feed-content :global(img) {
			max-width: 80%;
		}
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
		padding-bottom: 0.25rem;
	}
</style>
