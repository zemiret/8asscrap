<script lang="ts">
	import { PUBLIC_API_BASE_URL } from '$env/static/public';
	import { Heading, Input, P, Button } from 'flowbite-svelte';
	import AscentTypeCircle from '$lib/AscentTypeCircle.svelte';
	import { DownloadSolid, RefreshOutline } from 'flowbite-svelte-icons';
	import type { Ascent } from '../domain';
	import AscentRow from '$lib/AscentRow.svelte';

	const APIBaseURLV1 = PUBLIC_API_BASE_URL;

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
			zlaggableName: 'Masa krytyczna',
			countryName: 'Poland',
			cragName: 'Jura Krakowsko-Częstochowska',
			areaName: 'Dolina Szklarki',
			difficulty: '7a+',
			comment:
				'E-mej-zing! No góra to jest absolutny majstersztyk techniczny. Jak na Prostowaniu Piranii się zachwycałem, to zachwyt dał się ponownie tutaj poczuć. Za to na dole 2 całkiem fajne baldziki. Schowałem godność do kieszeni i wyrestowałem w dziurze z twarzą w pająkach i wypiętym dupskiem, a na wyjście na płytę znalazłem taki patent z lewą, a potem prawą piętą, że wręcz wydał mi się oszukańczy - takie się to to wtedy proste zrobiło. W sumie zawędzone ze 2 razy i 1sza próba na prowadzenie.',
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
			date: new Date(8.64e11)
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
			getAscentsErrorMsg = "Could not extract user name from the URL. Are you sure it's a valid logbook URL?";
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
			getAscentsErrorMsg = "Couldn't fetch user ascents. It looks like a problem on our side. Sorry :|";
			return;
		}
	}
</script>

<div class="max-w-10xl xl:grid xl:grid-cols-12">
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
					<Heading tag="h5" class="mb-4">These are the last ascents of {username} I know of:</Heading>

					<div class="mb-4">
						{#each lastAscents as ascent}
							<AscentRow {ascent} />
						{/each}
					</div>

					<!-- TODO: Make these die by side and pimp up the button -->

					<div class="flex">
						<P class="self-center">Not the newest?</P>
						<Button class="ml-4" on:click={reloadAscents} outline><RefreshOutline />Refresh ascents</Button>
					</div>
				{/if}

				{#if !!reloadAscentsErrorMsg}
					<p>{reloadAscentsErrorMsg}</p>
				{/if}
			{/if}
		</div>

		<div class="mb-4">
			<Heading tag="h4" class="mb-2">3. Experiment with the data</Heading>
			<!-- TODO: Advanced mode -->
			<p>Explanation what we can do with the data here</p>
			<p>And about redash, and about the parts underneath</p>
			<p>And a link to redash</p>
		</div>
	</div>
	<div class="col-span-3" />
</div>
