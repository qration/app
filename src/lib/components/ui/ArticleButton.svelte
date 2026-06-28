<script lang="ts">
	import type { Article } from '$lib/util/bindings';
	import { getRelativeTime } from '$lib/util/date';
	import { dateStrParse } from '$lib/util/util';
	import Icon from '@iconify/svelte';

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
	class="flex flex-col text-lg border-2 border-border items-start
		text-text-muted hover:text-text
		{selected ? 'bg-bg-hover' : 'bg-bg-secondary'} hover:bg-bg-hover rounded
		px-3 py-2 cursor-pointer min-w-0 max-w-full"
	{onclick}>
	<div class="font-medium truncate min-w-0 max-w-full">
		{article.read ? '' : '\u2022 '}{article.name}
	</div>
	<div class="flex flex-row justify-between w-full">
		<div class="flex flex-row gap-2 items-center">
			<Icon {icon} />
			<span class="truncate min-w-0 max-w-full">
				{author}
			</span>
		</div>
		{getRelativeTime(dateStrParse(article.date) * 1000, rtf)}
	</div>
</button>
