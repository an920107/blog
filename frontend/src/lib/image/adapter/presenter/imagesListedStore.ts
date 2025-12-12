import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';
import type { ListImagesUseCase } from '$lib/image/application/useCase/listImagesUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type ImageListState = AsyncState<readonly ImageInfoViewModel[]>;

export class ImagesListedStore implements BaseStore<ImageListState, void> {
	private readonly state = writable<ImageListState>(AsyncState.idle([]));

	constructor(
		private readonly listImagesUseCase: ListImagesUseCase,
		initialData?: readonly ImageInfoViewModel[]
	) {
		if (initialData) {
			this.state.set(AsyncState.idle(initialData));
		}
	}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return () => this.loadImages();
	}

	private async loadImages(): Promise<ImageListState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: ImageListState;
		try {
			const images = await this.listImagesUseCase.execute();
			const imageViewModels = images.map((image) => ImageInfoViewModel.fromEntity(image));
			result = AsyncState.success(imageViewModels);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}
