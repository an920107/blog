<script lang="ts">
	import type { ImageApiService } from '$lib/image/adapter/gateway/imageApiService';
	import { ImageRepositoryImpl } from '$lib/image/adapter/gateway/imageRepositoryImpl';
	import { ImageBloc } from '$lib/image/adapter/presenter/imageBloc';
	import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';
	import { UploadImageUseCase } from '$lib/image/application/useCase/uploadImageUseCase';
	import { ImageApiServiceImpl } from '$lib/image/framework/api/imageApiServiceImpl';
	import ImageManagementPage from '$lib/image/framework/ui/ImageManagementPage.svelte';
	import { setContext } from 'svelte';

	const imageApiService: ImageApiService = new ImageApiServiceImpl(fetch);
	const imageRepository: ImageRepository = new ImageRepositoryImpl(imageApiService);
	const uploadImageUseCase = new UploadImageUseCase(imageRepository);
	const imageBloc = new ImageBloc(uploadImageUseCase);

	setContext(ImageBloc.name, imageBloc);
</script>

<ImageManagementPage />
