<script lang="ts">
	import Icon from '@iconify/svelte';

	import { getRelativeTime } from '$lib/util/date';
	import { dateStrParse } from '$lib/util/util';

	import type { Article } from '$lib/util/bindings';

	let {
		article,
		author,
		icon,
		onclick,
		selected,
	}: {
		article: Article;
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
		class="truncate text-left {article.read ? 'font-regular' : 'font-bold'}
			max-w-full min-w-0">
		{article.read ? '' : '\u2022 '}{article.name}
	</div>
	{#if article.description}
		<div
			class="line-clamp-2 max-w-full min-w-0 text-left text-base
				text-text-secondary">
			{article.description}
		</div>
	{/if}
	<div class="flex w-full flex-row justify-between text-base">
		<div class="flex flex-row items-center gap-2">
			<Icon {icon} />
			<span class="max-w-full min-w-0 truncate">
				{author}
			</span>
		</div>
		{getRelativeTime(dateStrParse(article.date) * 1000, rtf)}
	</div>
</button>
