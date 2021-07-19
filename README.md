# solo2-gui

This is a tech demo for a Solo 2 GUI built with [Tauri][tauri] and [Svelte][svelte].

## Get started

Install the dependencies...

```bash
pnpm install
```

...then start development server:

```bash
pnpm tauri dev
```

This will take care of running both frontend and backend of your app with watch attached to both. That means whenever you change soething in `src` (svelte frontend code) or `src-tauri` (rust backend code), it will be automatically processed and hot reloaded. To finish dev/debug mode simply close the app window.

## Building and running in production mode

To create an optimised version of the app:

```bash
pnpm tauri build
```

This will create standalone app and installer in `src-tauri/target/release` directory.

## Useful links

-   [Tauri][tauri]
-   [Svelte][svelte]
-   [Sveltestrap][sveltestrap]

[tauri]: https://tauri.studio
[svelte]: https://svelte.dev
[sveltestrap]: https://sveltestrap.js.org

