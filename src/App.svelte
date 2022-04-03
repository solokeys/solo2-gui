<script lang="ts">

  import { invoke } from '@tauri-apps/api/tauri';

  let uuid = '<undefined>';
  invoke('get_uuid', {})
    .then(res => {
      console.log(`Got UUID ${uuid}`);
      uuid = res;
    })
    .catch((err) => console.error(err));

  const getUUID = async () => {
    invoke('get_uuid', {})
      .then(res => uuid = res)
      .catch(e => uuid = e)
  };

  // OATH
  let register_user: string = 'alice@trussed.dev';
  let register_secret: string = 'JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP';
  let registered_label: string = '';
  const register_totp = async () => {
    registered_label = await invoke('register_totp', {
      uuid,
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
      uuid,
      label: calculate_label,
    });
    console.log(`TOTP calculated:: ${calculated_totp}`);
  };

  let listed_totp = [];//: string[] = [];
  const list_totp = async () => {
    console.log('Listing credentials');
    listed_totp = await invoke('list_totp', {uuid: uuid});
  }


</script>

<main>
  <div>
    <button class="btn" on:click={getUUID}>Get UUID</button>
    <div>UUID: {uuid}</div>

    <h2>Register TOTP</h2>
    <form>
      <label for="register_user">TOTP User</label>
      <input
        type="text"
        id="register_user"
        bind:value={register_user}
        placeholder="e.g. alice@trussed.dev"
      />
    </form>
    <form>
      <label for="register_secret">TOTP Secret</label>
      <input
        type="text"
        id="register_secret"
        bind:value={register_secret}
        placeholder="e.g. JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP"
      />
    </form>
    <button class="btn" on:click={register_totp}>Register</button>
    <div>
      {#if registered_label.length !== 0}
        Registered label: {registered_label}
      {:else}
        No TOTP registered just yet.
      {/if}
    </div>

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
  </div>
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
