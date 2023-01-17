<script lang="ts">

  import { onMount } from 'svelte';
  import { themeChange } from 'theme-change';
  import Picker from '$components/Picker.svelte';
  import Router from 'svelte-spa-router';
  import Start from '$routes/Start.svelte';
  import Update from '$routes/Update.svelte';
  import Fido from '$routes/Fido.svelte';
  import Oath from '$routes/Oath.svelte';
  import Piv from '$routes/Piv.svelte';

  const routes = {
    '/': Start,

    // Using named parameters, with last being optional
    '/update': Update,
    '/fido': Fido,
    '/oath': Oath,
    '/piv': Piv,

    // Catch-all, must be last
    '*': Start,
  };

  onMount(() => {
    themeChange(false)
    // 👆 false parameter is required for svelte
  })

</script>

<main class="prose">
  <header class="flex items-center justify-between">
    <div class="flex">
      <div><a href="#/">Start</a></div>
      <div class="ml-2"><a href="#/update">Update</a></div>
      <div class="ml-6">Apps:</div>
      <div class="ml-2"><a href="#/fido">FIDO</a></div>
      <div class="ml-2"><a href="#/oath">OATH</a></div>
      <div class="ml-2"><a href="#/piv">PIV</a></div>
    </div>
    <select data-choose-theme>
      <option value="">Default</option>
      <option value="corporate">Corporate</option>
      <option value="cupcake">Cupcake</option>
      <option value="cyberpunk">Cyberpunk</option>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
      <option value="lemonade">Lemonade</option>
      <option value="retro">Retro</option>
    </select>
  </header>
  <Picker />
  <Router {routes} />
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
