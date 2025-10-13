<script lang="ts">
	import type { AuthApiService } from '$lib/auth/adapter/gateway/authApiService';
	import { AuthRepositoryImpl } from '$lib/auth/adapter/gateway/authRepositoryImpl';
	import { AuthBloc, AuthEventType } from '$lib/auth/adapter/presenter/authBloc';
	import type { AuthRepository } from '$lib/auth/application/gateway/authRepository';
	import { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';
	import { AuthApiServiceImpl } from '$lib/auth/framework/api/authApiServiceImpl';
	import { onMount, setContext } from 'svelte';
	import type { LayoutProps } from './$types';
	import { StatusType } from '$lib/common/adapter/presenter/asyncState';
	import ErrorPage from '$lib/common/framework/ui/ErrorPage.svelte';
	import DashboardNavbar from '$lib/dashboard/framework/ui/DashboardNavbar.svelte';
	import type { DashboardLink } from '$lib/dashboard/framework/ui/dashboardLink';

	const { children }: LayoutProps = $props();

	const authApiService: AuthApiService = new AuthApiServiceImpl(fetch);
	const authRepository: AuthRepository = new AuthRepositoryImpl(authApiService);
	const getcurrentUserUseCase = new GetCurrentUserUseCase(authRepository);
	const authBloc = new AuthBloc(getcurrentUserUseCase);

	setContext(AuthBloc.name, authBloc);

	onMount(() => authBloc.dispatch({ event: AuthEventType.CurrentUserLoadedEvent }));

	const authState = $derived($authBloc);
	const isLoading = $derived.by(
		() => authState.status === StatusType.Loading || authState.status === StatusType.Idle
	);
	const hasError = $derived.by(() => {
		if (authState.status === StatusType.Error) {
			return true;
		}
		if (authState.status === StatusType.Success && !authState.data.isAuthenticated) {
			return true;
		}
		return false;
	});

	const links: DashboardLink[] = [
		{ label: 'Post', href: '/dashboard/post' },
		{ label: 'Label', href: '/dashboard/label' },
		{ label: 'Image', href: '/dashboard/image' },
	];
</script>

{#if isLoading}
	<div></div>
{:else if hasError}
	<ErrorPage />
{:else}
	<div class="grid min-h-content-height grid-cols-[auto_1fr]">
		<DashboardNavbar {links} />
		{@render children()}
	</div>
{/if}
