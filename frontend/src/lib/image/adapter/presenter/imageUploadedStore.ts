import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';
import type { UploadImageUseCase } from '$lib/image/application/useCase/uploadImageUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type ImageInfoState = AsyncState<ImageInfoViewModel>;

export class ImageUploadedStore implements BaseStore<ImageInfoState, File> {
	private readonly state = writable<ImageInfoState>(AsyncState.idle<ImageInfoViewModel>(null));

	constructor(private readonly uploadImageUseCase: UploadImageUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (file: File) => this.uploadImage(file);
	}

	private async uploadImage(file: File): Promise<ImageInfoState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: ImageInfoState;
		try {
			const imageInfo = await this.uploadImageUseCase.execute(file);
			const imageInfoViewModel = ImageInfoViewModel.fromEntity(imageInfo);
			result = AsyncState.success(imageInfoViewModel);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}
