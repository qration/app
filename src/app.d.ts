import '@poppanator/sveltekit-svg/dist/svg.d.ts';

// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		interface PageState {
			showSidebar?: boolean;
			// Marks the entry pushed by opening the sidebar, so it can be popped.
			sidebarEntry?: boolean;
			showArticle?: boolean;
			selectedArticleId?: string;
			dialogOpen?: boolean;
		}
		// interface Platform {}
	}
}

export {};
