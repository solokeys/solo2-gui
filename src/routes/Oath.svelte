<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { uuid } from '$lib/stores.ts';

  let register_user: string = 'alice@trussed.dev';
  let register_secret: string = 'JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP';
  let registered_label: string = '';
  const register_totp = async () => {
    registered_label = await invoke('register_totp', {
      $uuid,
      label: register_user,
      secret: register_secret,
    });
    console.log(`TOTP registered with label: ${registered_label}`);
  };

  let calculate_label = 'alice@trussed.dev';
  let calculated_totp = '';
  const calculate_totp = async () => {
    console.log(`Calculating for label: ${calculate_label}`);
    calculated_totp = await invoke('calculate_totp', {
      uuid: $uuid,
      label: calculate_label,
    });
    console.log(`TOTP calculated:: ${calculated_totp}`);
  };

  let listed_totp = [];//: string[] = [];
  const list_totp = async () => {
    console.log('Listing credentials');
    listed_totp = await invoke('list_totp', {uuid: $uuid});
  }
</script>


<h2>List TOTP</h2>
<button class="btn" on:click={list_totp}>List</button>
<div>
  {#if listed_totp.length !== 0}
    <ul>
      {#each listed_totp as label}
      <li>{label}</li>
      {/each}
    </ul>
  {:else}
    No TOTP listed just yet.
  {/if}
</div>

<h2>Calculate TOTP</h2>
<form>
  <label for="calculate_label">TOTP Label</label>
  <input
    type="text"
    id="calculate_label"
    bind:value={calculate_label}
    placeholder="e.g. alice@trussed.dev"
    class="input input-sm input-bordered"
  />
</form>
<button class="btn" on:click={calculate_totp}>Calculate</button>
<div>
  {#if calculated_totp.length !== 0}
    Calculated TOTP: {calculated_totp}
  {:else}
    No TOTP generated just yet.
  {/if}
</div>

<h2>Register TOTP</h2>
<form>
  <label for="register_user">TOTP User</label>
  <input
    type="text"
    id="register_user"
    bind:value={register_user}
    placeholder="e.g. alice@trussed.dev"
    class="input input-sm input-bordered"
  />
</form>
<div class="form-control w-full max-w-xs">
  <label for="register_secret" class="label">
    <span class="label-text">TOTP Secret</span>
  </label>
  <input
    type="text"
    id="register_secret"
    bind:value={register_secret}
    placeholder="JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP"
    class="input input-bordered w-full max-w-xs"
  >
  <label for="register_secret" class="label">
    <span class="label-text-alt">Base32 encoded</span>
  </label>
</div>
<button class="btn btn-primary" on:click={register_totp}>Register</button>
<div>
  {#if registered_label.length !== 0}
    Registered label: {registered_label}
  {:else}
    No TOTP registered just yet.
  {/if}
</div>

