import { StatusType, type AsyncState } from '$lib/common/adapter/presenter/asyncState';
import { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';
import type { UploadImageUseCase } from '$lib/image/application/useCase/uploadImageUseCase';
import { get, writable } from 'svelte/store';

export type ImageInfoState = AsyncState<ImageInfoViewModel>;
export type ImageEvent = ImageUploadedEvent;

export class ImageBloc {
	private readonly state = writable<ImageInfoState>({
		status: StatusType.Idle,
	});

	constructor(private readonly uploadImageUseCase: UploadImageUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	async dispatch(event: ImageEvent): Promise<ImageInfoState> {
		switch (event.event) {
			case ImageEventType.ImageUploadedEvent:
				return this.uploadImage(event.file);
		}
	}

	private async uploadImage(file: File): Promise<ImageInfoState> {
		this.state.set({ status: StatusType.Loading, data: get(this.state).data });

		let result: ImageInfoState;
		try {
			const imageInfo = await this.uploadImageUseCase.execute(file);
			const imageInfoViewModel = ImageInfoViewModel.fromEntity(imageInfo);
			result = { status: StatusType.Success, data: imageInfoViewModel };
		} catch (error) {
			result = { status: StatusType.Error, error: error as Error };
		}

		return result;
	}
}

export enum ImageEventType {
	ImageUploadedEvent,
}

interface ImageUploadedEvent {
	event: ImageEventType.ImageUploadedEvent;
	file: File;
}
