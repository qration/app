<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import { addCollection } from '@iconify/svelte';
	import { icons } from '@iconify-json/tabler';
	import 'overlayscrollbars/overlayscrollbars.css';

	let { children } = $props();

	// TODO: add only the necessary icons here
	addCollection(icons);
	import { OverlayScrollbars } from 'overlayscrollbars';
	OverlayScrollbars.env().setDefaultOptions({
		scrollbars: { theme: 'os-theme-dark' },
	});
</script>

<ModeWatcher />

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
