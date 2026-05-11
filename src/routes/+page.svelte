<script lang="ts">
	import logo from '$lib/assets/qration.svg';
	import { resolve } from '$app/paths';
</script>

<div class="flex justify-center align-center h-screen w-screen">
	<img src={logo} alt="Logo" />
	<a href={resolve('/feed')}>Go to your feed</a>
</div>
