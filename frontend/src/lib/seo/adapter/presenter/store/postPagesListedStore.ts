import type { ListPostPagesUseCase } from '$lib/seo/application/useCase/listPostPagesUseCase';
import type { PostPage } from '$lib/seo/domain/entity/postPage';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class PostPagesListedStore extends BaseActionStore<readonly PostPage[]> {
	constructor(private readonly listPostPagesUseCase: ListPostPagesUseCase) {
		super();
	}

	protected async execute(): Promise<readonly PostPage[]> {
		return this.listPostPagesUseCase.execute();
	}
}
