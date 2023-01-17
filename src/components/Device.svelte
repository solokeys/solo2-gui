<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { uuid as uuidStore } from '$lib/stores.ts';

  export let uuid;

  const wink = () => {
    invoke('wink', {uuid})
      .then(s => {
        console.log("winked");
      })
      .catch((err) => console.error(err));
  }

  const toggle = () => {
    if (uuid == $uuidStore) {
      $uuidStore = "";
    } else {
      $uuidStore = uuid;
      console.log(`using ${uuid}`);
    }
  }
</script>

<div class="flex">
  <!--
    <button class="btn btn-xs btn-primary" on:click={use}>use</button>
  -->
  <div
  class:bg-accent="{$uuidStore === uuid}"
  class="flex-auto font-mono hover:bg-accent-focus"
  on:click={toggle}
  >{uuid}</div>
  <button class="btn btn-xs btn-secondary" on:click={wink}>wink</button>
</div>
