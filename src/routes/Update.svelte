<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from '@tauri-apps/api/tauri';
  import { emit, listen } from '@tauri-apps/api/event';
  import { appWindow } from '@tauri-apps/api/window';
  import { uuid } from '$lib/stores.ts';

  let firmware_position = 0;
  let firmware_size = 0;
  let prompt_touch = false;
  let updating = false;

  onMount(async () => {
    await appWindow.listen("prompt", async (event) => {
      console.log("confirm:", confirm);
      let confirmation = await confirm(event.payload.message);
      console.log("confirmation:", confirmation);
      if(confirmation) {
        appWindow.emit("reply", "ok");
      } else {
        appWindow.emit("reply", "cancel");
      };
    });
    await appWindow.listen("firmware-position", async (event) => {
      let position = event.payload;
      console.log("firmware-position:", position);
      firmware_position = position;
    });
    await appWindow.listen("touch-device", async (event) => {
      prompt_touch = event.payload;
    });
    await appWindow.listen("update-starts", async (event) => {
      prompt_touch = false;
      updating = true;
    });
    await appWindow.listen("firmware-size", async (event) => {
      let size = event.payload;
      console.log("firmware-size:", size);
      firmware_size = size;
    });
  });
  const update_device = async () => {
    console.log('Updating device', $uuid);

    await Promise.all([
      await appWindow.listen("prompt", async (event) => {
        console.log("confirm:", confirm);
        let confirmation = await confirm(event.payload.message);
        console.log("confirmation:", confirmation);
        if(confirmation) {
          appWindow.emit("reply", "ok");
        } else {
          appWindow.emit("reply", "cancel");
        };
      }),
      await invoke('update_device', {uuid: $uuid}),
    ])
    .then(([unlistener, response]) => {
      console.log("unlistener:", unlistener);
      unlistener();
      console.log("response:", response);
    })
    .catch(err => {
      console.log("err:", err)
    });
    updating = false;
  }
</script>

<h2>Update Device</h2>
{#if updating}
<p>Writing {firmware_size} bytes</p>
<progress class="progress progress-success w-56" value="{firmware_position}" max="{firmware_size}"></progress>
{/if}
{#if prompt_touch}
<p class="alert alert-info shadow-lg">Touch device to start update process!</p>
{/if}
{#if !(updating || prompt_touch) }
<button class="btn" disabled={$uuid == ""} on:click={update_device}>Update</button>
{/if}
