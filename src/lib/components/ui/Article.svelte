<script lang="ts">
	import { getRelativeTime } from '$lib/util/date';
	import Icon from '@iconify/svelte';

	let {
		title,
		author,
		time,
		icon,
		onclick,
		selected,
	}: {
		title: string;
		author: string;
		time: Date;
		icon: string;
		onclick?: (e: MouseEvent) => void;
		selected: boolean;
	} = $props();

	const rtf = new Intl.RelativeTimeFormat('en-CA');
</script>

<button
	class="flex flex-col text-lg border-2 border-border items-start
		text-text-muted hover:text-text {selected ? 'bg-bg-hover' : 'bg-bg'}
		hover:bg-bg-hover rounded px-3 py-2 cursor-pointer"
	{onclick}>
	<div class="font-medium">
		{title}
	</div>
	<div class="flex flex-row justify-between w-full">
		<div class="flex flex-row gap-2 items-center">
			<Icon {icon} />
			{author}
		</div>
		{getRelativeTime(time.getTime(), rtf)}
	</div>
</button>
