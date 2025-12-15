<script>
	import { Environment } from '$lib/environment';
</script>

<footer class="border-t border-gray-300">
	<div
		class="mx-auto flex max-w-screen-xl flex-col items-center justify-center gap-4 px-4 py-12 text-gray-600 md:flex-row md:px-6"
	>
		<div class="flex flex-row items-center justify-center gap-x-4">
			<a href="https://www.youtube.com/@squidspirit16" target="_blank" aria-label="YouTube Channel">
				<i class="fa-brands fa-youtube size-4" title="YouTube Channel"></i>
			</a>
			<a href="mailto:squid@squidspirit.com" aria-label="Email">
				<i class="fa-solid fa-envelope size-4" title="Email"></i>
			</a>
			<a href="{Environment.APP_BASE_URL}feed.xml" target="_blank" aria-label="RSS Feed">
				<i class="fa-solid fa-square-rss"></i>
			</a>
			<a href="https://git.squidspirit.com/squid/blog" target="_blank" aria-label="Git Repository">
				<i class="fa-brands fa-git-alt size-4" title="Git Repository"></i>
			</a>
		</div>
		<div class="max-md:hidden">
			<div class="h-4 w-0.5 bg-gray-300"></div>
		</div>
		<span class="text-sm">Copyright © 2025 SquidSpirit</span>
	</div>
</footer>
