<script lang="ts">
	import { PUBLIC_API_BASE_URL } from '$env/static/public';
	import { Heading, P } from 'flowbite-svelte';

	const APIBaseURLV1 = PUBLIC_API_BASE_URL;

	interface Ascent {
		type: 'f' | 'os' | 'rp';
		zlaggableName: string;
		countryName: String;
		cragName: string;
		areaName?: string;
		difficulty: string;
		comment?: string;
		date: Date;
	}

	// TODO: Default link just for development - to not have to input it every time
	// let logbookLink: string = 'https://www.8a.nu/user/ania-w/sportclimbing';
	let logbookLink: string;
	let username: string | undefined;
	let getAscentsErrorMsg: string;
	let reloadAscentsErrorMsg: string;
	let lastAscents: Ascent[] | null = null;

	async function getLastAscents() {
		getAscentsErrorMsg = '';
		reloadAscentsErrorMsg = '';
		lastAscents = null;

		if (logbookLink == null) {
			getAscentsErrorMsg = 'Link was not provided.';
			return;
		}

		username = logbookLink.split('/').at(4);
		if (username == null) {
			getAscentsErrorMsg =
				"Could not extract user name from the URL. Are you sure it's a valid logbook URL?";
			return;
		}

		loadLastAscents(username);
	}

	async function reloadAscents() {
		if (username == null) {
			return; // sth would need to be very wrong for this to happen
		}

		reloadAscentsErrorMsg = '';

		try {
			const res = await fetch(`${APIBaseURLV1}/ascents/${username}/reload`, {
				method: 'POST'
			});
			if (!res.ok) {
				reloadAscentsErrorMsg = "Couldn't load user ascents. Try again.";
				return;
			}
			lastAscents = await res.json();
		} catch (err) {
			console.log(`Fetch POST reload ascents err: ${err}`);
		}

		loadLastAscents(username);
	}

	async function loadLastAscents(username: string) {
		try {
			const res = await fetch(`${APIBaseURLV1}/ascents/${username}/last`);
			if (!res.ok) {
				getAscentsErrorMsg = "Couldn't fetch user ascents. Try again.";
				return;
			}
			lastAscents = await res.json();
		} catch (err) {
			console.log(`Fetch last ascents err: ${err}`);
		}
	}
</script>

<div class="m-8 max-w-40">
	<Heading tag="h1">Paste logbook URL</Heading>
	<P>Whatever elo</P>

	<input placeholder="https://www.8a.nu/user/adam-ondra/sportclimbing" bind:value={logbookLink} />
	<button type="button" disabled={!logbookLink} on:click={getLastAscents}>Get ascents</button>
	{#if !!getAscentsErrorMsg}
		<p>{getAscentsErrorMsg}</p>
	{/if}

	{#if !(lastAscents === null)}
		{#if lastAscents.length === 0}
			<p>Last ascents empty</p>
			<button type="button" on:click={reloadAscents}>Load ascents</button>
		{:else}
			{#each lastAscents as ascent}
				<p>{ascent.type}</p>
				<p>{ascent.zlaggableName}</p>
				<p>{ascent.countryName}</p>
				<p>{ascent.cragName}</p>
				<p>{ascent.areaName}</p>
				<p>{ascent.difficulty}</p>
				<p>{ascent.comment}</p>
				<p>{ascent.date}</p>
			{/each}

			<button type="button" on:click={reloadAscents}>Reload ascents</button>
		{/if}

		{#if !!reloadAscentsErrorMsg}
			<p>{reloadAscentsErrorMsg}</p>
		{/if}
	{/if}

	<!-- Advanced mode -->
	<p>Explanation what we can do with the data here</p>
	<p>And about redash, and about the parts underneath</p>
	<p>And a link to redash</p>
</div>

<style>
	input {
		width: 100%;
	}
</style>
