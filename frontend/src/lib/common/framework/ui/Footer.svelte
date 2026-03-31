<script lang="ts">
	import { Strings } from '$lib/strings';
	import { cn } from '$lib/common/framework/components/utils';
	import { Links } from '$lib/links';

	const currentYear = new Date().getFullYear();
</script>

<footer class="border-t border-gray-300 text-center text-sm text-gray-600">
	<div class="md:hidden">
		{@render mobileLayout()}
	</div>
	<div class="max-md:hidden">
		{@render desktopLayout()}
	</div>
</footer>

{#snippet desktopLayout()}
	<div class="mx-auto flex max-w-screen-xl flex-row items-center justify-center gap-12 px-10 py-12">
		{@render icons()}
		{@render divider()}
		<div class="flex flex-col items-start gap-4">
			{@render policies('start')}
			{@render copyright()}
		</div>
	</div>
{/snippet}

{#snippet mobileLayout()}
	<div class="mx-auto flex flex-col items-center justify-center gap-8 px-4 py-12">
		{@render icons()}
		{@render policies('center')}
		{@render copyright()}
	</div>
{/snippet}

{#snippet icons()}
	<div class="flex flex-row items-center justify-center gap-x-4">
		<a href={Links.YOUTUBE} target="_blank" aria-label="YouTube Channel">
			<i class="fa-brands fa-youtube text-[1rem]" title="YouTube Channel"></i>
		</a>
		<a href={Links.EMAIL} aria-label="Email">
			<i class="fa-solid fa-envelope text-[1rem]" title="Email"></i>
		</a>
		<a href={Links.RSS} target="_blank" aria-label="RSS Feed">
			<i class="fa-solid fa-square-rss text-[1rem]"></i>
		</a>
		<a href={Links.SOURCE_CODE} target="_blank" aria-label="Git Repository">
			<i class="fa-brands fa-git-alt text-[1.05rem]" title="Git Repository"></i>
		</a>
	</div>
{/snippet}

{#snippet policies(align: 'center' | 'start')}
	<div class={cn('flex flex-col justify-center gap-1', `items-${align}`)}>
		<a href={Links.PRIVACY_POLICY}>{Strings.PRIVACY_POLICY}</a>
		<a href={Links.AI_USAGE_POLICY}>{Strings.AI_USAGE_POLICY}</a>
	</div>
{/snippet}

{#snippet copyright()}
	<span>Copyright © {currentYear} SquidSpirit</span>
{/snippet}

{#snippet divider()}
	<div class="h-20 w-0.5 bg-gray-300"></div>
{/snippet}
