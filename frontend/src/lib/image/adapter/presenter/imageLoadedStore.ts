import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';
import type { GetImageInfoUseCase } from '$lib/image/application/useCase/getImageInfoUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type ImageState = AsyncState<ImageInfoViewModel>;

export class ImageLoadedStore implements BaseStore<ImageState, number> {
	private readonly state = writable<ImageState>(AsyncState.idle(null as ImageInfoViewModel | null));

	constructor(
		private readonly getImageInfoUseCase: GetImageInfoUseCase,
		initialData?: ImageInfoViewModel
	) {
		if (initialData) {
			this.state.set(AsyncState.success(initialData));
		}
	}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (id: number) => this.loadImage(id);
	}

	private async loadImage(id: number): Promise<ImageState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: ImageState;
		try {
			const image = await this.getImageInfoUseCase.execute(id);
			const viewModel = ImageInfoViewModel.fromEntity(image);
			result = AsyncState.success(viewModel);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}
