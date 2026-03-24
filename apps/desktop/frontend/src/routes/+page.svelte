<script lang="ts">
	import { Button } from '$lib/components/ui/button';
</script>

<div class="min-h-screen bg-background p-12">
	<h1 class="text-foreground mb-2 text-3xl font-bold">shadcn-svelte Button</h1>
	<p class="text-muted-foreground mb-10 text-sm">All variants and sizes</p>

	<section class="mb-10">
		<h2 class="text-foreground mb-4 text-sm font-semibold tracking-wide uppercase">Variants</h2>
		<div class="flex flex-wrap gap-3">
			<Button variant="default">Default</Button>
			<Button variant="secondary">Secondary</Button>
			<Button variant="outline">Outline</Button>
			<Button variant="ghost">Ghost</Button>
			<Button variant="destructive">Destructive</Button>
			<Button variant="link">Link</Button>
		</div>
	</section>

	<section class="mb-10">
		<h2 class="text-foreground mb-4 text-sm font-semibold tracking-wide uppercase">Sizes</h2>
		<div class="flex flex-wrap items-center gap-3">
			<Button size="xs">Extra Small</Button>
			<Button size="sm">Small</Button>
			<Button size="default">Default</Button>
			<Button size="lg">Large</Button>
		</div>
	</section>

	<section>
		<h2 class="text-foreground mb-4 text-sm font-semibold tracking-wide uppercase">States</h2>
		<div class="flex flex-wrap gap-3">
			<Button>Enabled</Button>
			<Button disabled>Disabled</Button>
			<Button href="https://shadcn-svelte.com" variant="link">As anchor</Button>
		</div>
	</section>
</div>
