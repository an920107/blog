import { dev } from '$app/environment';
import { type AuthModule, createAuthModule } from '$lib/auth/composition';
import { Environment } from '$lib/environment';
import type { ImageInfoViewModel } from '$lib/image/adapter/presenter/viewModel/imageInfoViewModel';
import { createImageModule, type ImageModule } from '$lib/image/composition';
import type { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
import { createLabelModule, type LabelModule } from '$lib/label/composition';
import type { PostInfoViewModel } from '$lib/post/adapter/presenter/viewModel/postInfoViewModel';
import type { PostViewModel } from '$lib/post/adapter/presenter/viewModel/postViewModel';
import { createPostModule, type PostModule } from '$lib/post/composition';
import { createSeoModule, type SeoModule } from '$lib/seo/composition';
import { DrawerConfiguredStore } from '$lib/shared/adapter/presenter/store/drawerConfiguredStore';
import { ApiClient } from '$lib/shared/framework/http/apiClient';
import { FetchHttpClient } from '$lib/shared/framework/http/fetchHttpClient';
import { createMockFetch } from '$lib/shared/framework/mock/mockApiFetch';

export class Container {
	private readonly auth: AuthModule;
	private readonly image: ImageModule;
	private readonly label: LabelModule;
	private readonly post: PostModule;
	private readonly seo: SeoModule;

	constructor(fetchFn: typeof fetch) {
		const effectiveFetch = dev && Environment.USE_MOCK_API ? createMockFetch(fetchFn) : fetchFn;
		const apiClient = new ApiClient(new FetchHttpClient(effectiveFetch), Environment.API_BASE_URL);

		this.auth = createAuthModule(apiClient);
		this.image = createImageModule(apiClient);
		this.label = createLabelModule(apiClient);
		this.post = createPostModule(apiClient);
		this.seo = createSeoModule(this.post);
	}

	createDrawerConfiguredStore(): DrawerConfiguredStore {
		return new DrawerConfiguredStore();
	}

	createAuthLoadedStore() {
		return this.auth.createAuthLoadedStore();
	}

	createImageUploadedStore() {
		return this.image.createImageUploadedStore();
	}

	createImagesListedStore(initialData?: readonly ImageInfoViewModel[]) {
		return this.image.createImagesListedStore(initialData);
	}

	createImageLoadedStore(initialData?: ImageInfoViewModel) {
		return this.image.createImageLoadedStore(initialData);
	}

	createImageDeletedStore() {
		return this.image.createImageDeletedStore();
	}

	createPostsListedStore(initialData?: readonly PostInfoViewModel[]) {
		return this.post.createPostsListedStore(initialData);
	}

	createPostLoadedStore(initialData?: PostViewModel) {
		return this.post.createPostLoadedStore(initialData);
	}

	createPostCreatedStore() {
		return this.post.createPostCreatedStore();
	}

	createPostUpdatedStore() {
		return this.post.createPostUpdatedStore();
	}

	createPostPagesListedStore() {
		return this.seo.createPostPagesListedStore();
	}

	createLabelsListedStore(initialData?: readonly LabelViewModel[]) {
		return this.label.createLabelsListedStore(initialData);
	}

	createLabelLoadedStore(initialData?: LabelViewModel) {
		return this.label.createLabelLoadedStore(initialData);
	}

	createLabelCreatedStore() {
		return this.label.createLabelCreatedStore();
	}

	createLabelUpdatedStore() {
		return this.label.createLabelUpdatedStore();
	}
}
