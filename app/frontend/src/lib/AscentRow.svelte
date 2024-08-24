<script lang="ts">
	import { P } from 'flowbite-svelte';
	import type { Ascent } from '../domain';
	import AscentTypeCircle from './AscentTypeCircle.svelte';

	export let ascent: Ascent;

	function ascentFormatDate(date: Date) {
		let month = date.toLocaleString('en-US', { month: 'short' });
		let day = date.getDay();
		let year = date.getFullYear();

		return `${day} ${month} ${year}`;
	}
</script>

<!-- TODO: Let's do ascents as a small component for now same as they're on the small screen. To support bigger screen we could switch to another grid layout conditionally on 2xl size -->
<div class="ascent-row mb-2 grid grid-cols-12 gap-4 pb-2">
	<div class="col-span-12 flex">
		<div class="mr-2 self-center">
			<AscentTypeCircle type={ascent.type} />
		</div>

		<div>
			<P size="lg">{ascent.zlaggableName} ({ascent.difficulty})</P>
			<P size="xs">
				{ascent.countryName}, {ascent.cragName}
				{#if !!ascent.areaName}
					, {ascent.areaName}
				{/if}
			</P>
		</div>
	</div>

	<div class="col-span-12">
		{#if !!ascent.comment}
			<P>{ascent.comment}</P>
		{/if}
	</div>

	<div class="col-span-2 col-start-10 justify-self-end">
		<P size="xs">{ascentFormatDate(ascent.date)}</P>
	</div>
</div>

<style>
	.ascent-row {
		border-bottom: 2px solid #f2f2ef;
	}
</style>
