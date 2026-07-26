<script lang="ts">
	import Icon from '@iconify/svelte';

	import { getRelativeTime } from '$lib/util/date';

	import type { ArticleLight } from '$lib/util/bindings';
	import { getNow } from '$lib/context/context.svelte';

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

	const rtf = new Intl.RelativeTimeFormat('en-CA', {
		style: 'narrow',
	});
	let rel = $derived(getRelativeTime(article.article_date, rtf, getNow()));
</script>

<button
	class="relative flex flex-row gap-4
		text-lg text-text-muted hover:text-text
		{selected
		? 'bg-bg-hover inset-shadow-[6px_0]'
		: 'bg-bg-secondary'} max-w-full min-w-0
		cursor-pointer border-b border-border px-4 py-3 inset-shadow-text hover:bg-bg-hover"
	{onclick}>
	<div
		class="mt-2.5 ml-1 shrink-0 {article.article_read
			? 'bg-transparent'
			: 'bg-text'} h-2 w-2 rounded-4xl">
	</div>
	<div class="flex w-full min-w-0 flex-col gap-1">
		<div
			class="truncate text-left {article.article_read
				? 'font-regular'
				: 'font-bold'}
				max-w-full min-w-0">
			{article.article_name}
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
				{rel}
			</div>
		</div>
	</div>
</button>
