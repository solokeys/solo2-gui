import App from './App.svelte';
import './app.css';

const app = new App({
	target: document.body,
	// target: document.getElementById('app'),
	props: {
		name: 'tauri',
	},
});

export default app;
