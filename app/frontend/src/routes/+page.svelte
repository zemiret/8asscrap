<script lang="ts">
	import { PUBLIC_API_BASE_URL } from '$env/static/public';
	import { Heading, Input, P, Button } from 'flowbite-svelte';
	import { DownloadSolid, RefreshOutline } from 'flowbite-svelte-icons';

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
	let logbookLink: string = 'https://www.8a.nu/user/ania-w/sportclimbing';
	// let logbookLink: string;
	let username: string | undefined;
	let getAscentsErrorMsg: string;
	let reloadAscentsErrorMsg: string;
	// TODO: Example ascents only for development
	let lastAscents: Ascent[] | null = [
		{
			type: 'os',
			zlaggableName: 'zlaggableName',
			countryName: 'countryName',
			cragName: 'cragName',
			areaName: 'areaName',
			difficulty: '7a+',
			comment:
				'commentcommentcomment comment commentcomment commentcommentcomment comment commentcomment commentcommentcomment comment commentcomment commentcommentcomment comment commentcomment',
			date: new Date()
		},
		{
			type: 'f',
			zlaggableName: 'zlaggableName',
			countryName: 'countryName',
			cragName: 'cragName',
			difficulty: '8b',
			date: new Date()
		},
		{
			type: 'rp',
			zlaggableName: 'zlaggableName',
			countryName: 'countryName',
			cragName: 'cragName',
			difficulty: '6a+',
			date: new Date(8.64e15)
		}
	];
	// let lastAscents: Ascent[] | null = null;

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
			getAscentsErrorMsg =
				"Couldn't fetch user ascents. It looks like a problem on our side. Sorry :|";
			return;
		}
	}
</script>

<div class="max-w-10xl lg:grid lg:grid-cols-12">
	<div class="col-span-3" />
	<div class="col-span-6">
		<Heading class="mb-8" tag="h1">Welcome to [figure out a name lol]</Heading>

		<div class="mb-4">
			<Heading tag="h4" class="mb-2">1. Paste logbook URL</Heading>
			<Input
				type="text"
				id="logbook-link"
				placeholder="https://www.8a.nu/user/adam-ondra/sportclimbing"
				bind:value={logbookLink}
			/>
		</div>

		<div class="mb-4">
			<Heading tag="h4" class="mb-2">2. Get ascents</Heading>
			<Button disabled={!logbookLink} on:click={getLastAscents}>
				<DownloadSolid class="me-2 h-5 w-5" />
				Get Ascents
			</Button>

			{#if !!getAscentsErrorMsg}
				<P class="mt-2 text-red-600">{getAscentsErrorMsg}</P>
			{/if}
		</div>

		<div class="mb-4">
			{#if !(lastAscents === null)}
				{#if lastAscents.length === 0}
					<!-- TODO: This section -->

					<p>Last ascents empty</p>
					<button type="button" on:click={reloadAscents}>Load ascents</button>
				{:else}
					<Heading tag="h5" class="mb-2">These are the last ascents of that user I know of:</Heading
					>

					{#each lastAscents as ascent}
						<div class="grid grid-cols-12">
							<!-- TODO: Style the ascents to look like 8a.nu ascents -->
						</div>

						<p>{ascent.type}</p>
						<p>{ascent.zlaggableName}</p>
						<p>{ascent.countryName}</p>
						<p>{ascent.cragName}</p>
						<p>{ascent.areaName}</p>
						<p>{ascent.difficulty}</p>
						<p>{ascent.comment}</p>
						<p>{ascent.date}</p>
					{/each}

					<!-- TODO: Make these die by side and pimp up the button -->
					<P class="my-2">Not the newest?</P>
					<Button on:click={reloadAscents} outline><RefreshOutline />Refresh ascents</Button>
				{/if}

				{#if !!reloadAscentsErrorMsg}
					<p>{reloadAscentsErrorMsg}</p>
				{/if}
			{/if}
		</div>

		<!-- TODO: Advanced mode -->
		<p>Explanation what we can do with the data here</p>
		<p>And about redash, and about the parts underneath</p>
		<p>And a link to redash</p>
	</div>
	<div class="col-span-3" />
</div>

<style>
	input {
		width: 100%;
	}
</style>
