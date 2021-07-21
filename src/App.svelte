<script lang="ts">
	import {
		Button,
		Card,
		CardBody,
		CardFooter,
		CardHeader,
		CardSubtitle,
		CardText,
		CardTitle,
		Container,
		FormGroup,
		Input,
		Label,
	} from 'sveltestrap';

	import { invoke } from '@tauri-apps/api/tauri';

	let input: string = '12';
	let result: string = '';
	let uuid: string = '<undefined>';
	invoke('get_uuid', {})
		.then((res) => {
			console.log(`Got UUID ${uuid}`);
			uuid = res;
		})
		.catch((err) => console.error(err));

	const getUUID = async () => {
		invoke('get_uuid', {})
			.then((res) => uuid = res)
			.catch((e) => uuid = e)
	};
	const handleClick = async () => {
		result = await invoke('generate_password', {
			length: +input,
		});
	};

	// OATH
	let register_user: string = 'alice@trussed.dev';
	let register_secret: string = 'JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP';
	let registered_label: string = '';
	const register_totp = async () => {
		registered_label = await invoke('register_totp', {
			user: register_user,
			secret: register_secret,
		});
		console.log(`TOTP registered with label: ${registered_label}`);
	};

	let calculate_label: string = 'alice@trussed.dev';
	let calculated_totp: string = '';
	const calculate_totp = async () => {
		console.log(`Calculating for label: ${calculate_label}`);
		calculated_totp = await invoke('calculate_totp', {
			label: calculate_label,
		});
		console.log(`TOTP calculated:: ${calculated_totp}`);
	};

  let listed_totp: string[] = [];
  const list_totp = async () => {
    console.log('Listing credentials');
    listed_totp = await invoke('list_totp', {});
  }


</script>

<main>
	<Container>
		<Card class="mb-3">
			<CardHeader>
				<CardTitle>Solo 2 TOTP</CardTitle>
			</CardHeader>
			<CardBody>
				<Button color="primary" on:click={getUUID}>Get UUID</Button>
				<CardFooter>UUID: {uuid}</CardFooter>

				<CardSubtitle>Register TOTP</CardSubtitle>
				<FormGroup>
					<Label for="register_user">TOTP User</Label>
					<Input
						type="text"
						id="register_user"
						bind:value={register_user}
						placeholder="e.g. alice@trussed.dev"
					/>
				</FormGroup>
				<FormGroup>
					<Label for="register_secret">TOTP Secret</Label>
					<Input
						type="text"
						id="register_secret"
						bind:value={register_secret}
						placeholder="e.g. JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP"
					/>
				</FormGroup>
				<Button color="primary" on:click={register_totp}>Register</Button>
				<CardFooter>
					{#if registered_label.length !== 0}
						Registered label: {registered_label}
					{:else}
						No TOTP registered just yet.
					{/if}
				</CardFooter>

				<CardSubtitle>List TOTP</CardSubtitle>
				<Button color="primary" on:click={list_totp}>List</Button>
				<CardFooter>
					{#if listed_totp.length !== 0}
            <ul>
              {#each listed_totp as label}
              <li>{label}</li>
              {/each}
            </ul>
          {:else}
						No TOTP listed just yet.
					{/if}

				</CardFooter>

				<CardSubtitle>Calculate TOTP</CardSubtitle>
				<FormGroup>
					<Label for="calculate_label">TOTP Label</Label>
					<Input
						type="text"
						id="calculate_label"
						bind:value={calculate_label}
						placeholder="e.g. alice@trussed.dev"
					/>
				</FormGroup>
				<Button color="primary" on:click={calculate_totp}>Calculate</Button>
				<CardFooter>
					{#if calculated_totp.length !== 0}
						Calculated TOTP: {calculated_totp}
					{:else}
						No TOTP generated just yet.
					{/if}
				</CardFooter>
			</CardBody>
			<CardFooter/>
		</Card>
	</Container>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
