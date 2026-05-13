<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/favicon.svg';
	import { ModeWatcher } from 'mode-watcher';
	import { addCollection } from '@iconify/svelte';
	import { icons } from '@iconify-json/tabler';

	let { children } = $props();

	// TODO: add only the necessary icons here
	addCollection(icons);
</script>

<ModeWatcher />

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
