import type { PostModule } from '$lib/post/composition';
import { PostPagesListedStore } from '$lib/seo/adapter/presenter/store/postPagesListedStore';
import { ListPostPagesUseCase } from '$lib/seo/application/useCase/listPostPagesUseCase';

export function createSeoModule(postModule: PostModule) {
	const listPostPagesUseCase = new ListPostPagesUseCase(postModule.getAllPostsUseCase);

	return {
		createPostPagesListedStore: () => new PostPagesListedStore(listPostPagesUseCase),
	};
}

export type SeoModule = ReturnType<typeof createSeoModule>;
