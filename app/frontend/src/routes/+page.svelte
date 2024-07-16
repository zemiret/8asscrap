<script lang="ts">
	// TODO: Will have to have dev vs prod different configuration
	const APIBaseURLV1 = 'localhost:8080/api/v1';

	// TODO: Default link just for development - to not have to input it every time
	let logbookLink: string = 'https://www.8a.nu/user/ania-w/sportclimbing';
	let username: string | undefined;
	let getAscentsErrorMsg: string;
	let lastAscents: Map<string, any>;

	async function getLastAscents() {
		getAscentsErrorMsg = '';

		if (logbookLink == null) {
			getAscentsErrorMsg = 'Link was not provided.';
			return;
		}

		username = logbookLink.split('/').at(4);
		if (username == null) {
			getAscentsErrorMsg =
				"Could not extract user name from the URL. Are you sure it's a valid URL?";
			return;
		}

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

	function reloadAscents() {}
</script>

<h1>1. Paste logbook URL</h1>

<input placeholder="https://www.8a.nu/user/adam-ondra/sportclimbing" bind:value={logbookLink} />
<button type="button" disabled={!logbookLink} on:click={getLastAscents}>Get ascents</button>
{#if !!getAscentsErrorMsg}
	<p>{getAscentsErrorMsg}</p>
{/if}

{#if !!lastAscents}
	<p>{lastAscents}</p>
{/if}

<!-- Advanced mode -->

<style>
	input {
		width: 100%;
	}
</style>
