<script lang="ts">
	import Icon from '@iconify/svelte';

	import { getRelativeTime } from '$lib/util/date';

	import type { ArticleLight } from '$lib/util/bindings';

	let {
		article,
		author,
		icon,
		onclick,
		selected,
	}: {
		article: ArticleLight;
		author: string;
		icon: string;
		onclick?: (e: MouseEvent) => void;
		selected: boolean;
	} = $props();

	const rtf = new Intl.RelativeTimeFormat('en-CA');
</script>

<button
	class="flex flex-col items-start gap-1 border-2 border-border
		text-lg text-text-muted hover:text-text
		{selected ? 'bg-bg-hover' : 'bg-bg-secondary'} max-w-full min-w-0
		cursor-pointer rounded px-3 py-2 hover:bg-bg-hover"
	{onclick}>
	<div
		class="truncate text-left {article.article_read
			? 'font-regular'
			: 'font-bold'}
			max-w-full min-w-0">
		{article.article_read ? '' : '\u2022 '}{article.article_name}
	</div>
	{#if article.article_description}
		<div
			class="line-clamp-2 max-w-full min-w-0 text-left text-base
				text-text-secondary">
			{article.article_description}
		</div>
	{/if}
	<div
		class="flex w-full flex-row justify-between gap-5 overflow-hidden text-base">
		<div class="flex flex-row items-center gap-2 truncate">
			<Icon {icon} class="shrink-0" />
			<span class="max-w-full min-w-0 truncate">
				{author}
			</span>
		</div>
		<div class="shrink-0">
			{getRelativeTime(article.article_date * 1000, rtf)}
		</div>
	</div>
</button>
