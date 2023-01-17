<script lang="ts">
  import { onMount } from 'svelte';
  import { get } from "svelte/store";
  import { invoke } from '@tauri-apps/api/tauri';
  import { uuid } from '$lib/stores.ts';
  import Device from '$components/Device.svelte';

  let uuids = [];

  const updateUuids = () => {
    invoke('get_uuids', {})
      .then(newUuids => {
        /* console.log(`Got UUIDs ${newUuids}`); */
        uuids = newUuids;
        if (!newUuids.includes(get(uuid))) {
          console.log("setting uuid to empty");
          $uuid = "";
        }
      })
      .catch((err) => console.error(err));
    /* console.log(uuids); */
  }
	onMount(async () => {
    updateUuids();
    setInterval(updateUuids, 5000);
	});

</script>

<h3>Devices</h3>
{#each uuids as uuid}
<Device {uuid}/>
{/each}
