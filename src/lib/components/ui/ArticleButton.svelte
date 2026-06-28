<script lang="ts">
	import { getRelativeTime } from '$lib/util/date';
	import Icon from '@iconify/svelte';

	let {
		title,
		author,
		timestamp,
		icon,
		onclick,
		selected,
	}: {
		title: string;
		author: string;
		timestamp: number;
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
		{title}
	</div>
	<div class="flex flex-row justify-between w-full">
		<div class="flex flex-row gap-2 items-center">
			<Icon {icon} />
			<span class="truncate min-w-0 max-w-full">
				{author}
			</span>
		</div>
		{getRelativeTime(timestamp * 1000, rtf)}
	</div>
</button>
