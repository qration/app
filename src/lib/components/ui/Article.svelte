<script lang="ts">
	import { getRelativeTime } from '$lib/util/date';

	let {
		title,
		author,
		time,
		iconClass,
		onclick,
	}: {
		title: string;
		author: string;
		time: Date;
		iconClass: string;
		onclick?: (e: MouseEvent) => void;
	} = $props();

	const rtf = new Intl.RelativeTimeFormat('en-CA');
</script>

<button
	class="flex flex-col text-lg border-2 border-border items-start
		text-text-muted hover:text-text bg-bg hover:bg-bg-hover rounded-xl px-3 py-2
		cursor-pointer"
	{onclick}>
	<div class="font-medium">
		{title}
	</div>
	<div class="flex flex-row justify-between w-full">
		<div class="flex flex-row gap-2 items-center">
			<i class={iconClass}></i>
			{author}
		</div>
		{getRelativeTime(time.getTime(), rtf)}
	</div>
</button>
