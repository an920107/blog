import type { AuthApiService } from '$lib/auth/adapter/gateway/authApiService';
import { AuthRepositoryImpl } from '$lib/auth/adapter/gateway/authRepositoryImpl';
import { AuthLoadedStore } from '$lib/auth/adapter/presenter/authLoadedStore';
import type { AuthRepository } from '$lib/auth/application/gateway/authRepository';
import { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';
import { AuthApiServiceImpl } from '$lib/auth/framework/api/authApiServiceImpl';
import type { LabelApiService } from '$lib/label/adapter/gateway/labelApiService';
import { LabelRepositoryImpl } from '$lib/label/adapter/gateway/labelRepositoryImpl';
import { LabelCreatedStore } from '$lib/label/adapter/presenter/labelCreatedStore';
import { LabelUpdatedStore } from '$lib/label/adapter/presenter/labelUpdatedStore';
import type { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
import type { LabelRepository } from '$lib/label/application/gateway/labelRepository';
import { CreateLabelUseCase } from '$lib/label/application/useCase/createLabelUseCase';
import { GetImageInfoUseCase } from '$lib/image/application/useCase/getImageInfoUseCase';
import { GetAllLabelsUseCase } from '$lib/label/application/useCase/getAllLabelsUseCase';
import { UpdateLabelUseCase } from '$lib/label/application/useCase/updateLabelUseCase';
import { LabelApiServiceImpl } from '$lib/label/framework/api/labelApiServiceImpl';
import type { ImageApiService } from '$lib/image/adapter/gateway/imageApiService';
import { ImageRepositoryImpl } from '$lib/image/adapter/gateway/imageRepositoryImpl';
import { ImageLoadedStore } from '$lib/image/adapter/presenter/imageLoadedStore';
import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';
import { ImagesListedStore } from '$lib/image/adapter/presenter/imagesListedStore';
import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';
import { UploadImageUseCase } from '$lib/image/application/useCase/uploadImageUseCase';
import { ListImagesUseCase } from '$lib/image/application/useCase/listImagesUseCase';
import { ImageApiServiceImpl } from '$lib/image/framework/api/imageApiServiceImpl';
import type { ImageInfoViewModel } from '$lib/image/adapter/presenter/imageInfoViewModel';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
import { PostCreatedStore } from '$lib/post/adapter/presenter/postCreatedStore';
import { PostUpdatedStore } from '$lib/post/adapter/presenter/postUpdatedStore';
import type { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
import type { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
import type { PostRepository } from '$lib/post/application/gateway/postRepository';
import { CreatePostUseCase } from '$lib/post/application/useCase/createPostUseCase';
import { UpdatePostUseCase } from '$lib/post/application/useCase/updatePostUseCase';
import { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';
import { GetLabelUseCase } from '$lib/label/application/useCase/getLabelUseCase';
import { LabelLoadedStore } from '$lib/label/adapter/presenter/labelLoadedStore';
import { DrawerConfiguredStore } from './common/adapter/presenter/drawerConfiguredStore';

export class Container {
	private useCases: UseCases;

	constructor(fetchFn: typeof fetch) {
		const apiServices = new ApiServices(fetchFn);
		const repositories = new Repositories(apiServices);
		this.useCases = new UseCases(repositories);
	}

	createDrawerConfiguredStore(): DrawerConfiguredStore {
		return new DrawerConfiguredStore();
	}

	createAuthLoadedStore(): AuthLoadedStore {
		return new AuthLoadedStore(this.useCases.getCurrentUserUseCase);
	}

	createImageUploadedStore(): ImageUploadedStore {
		return new ImageUploadedStore(this.useCases.uploadImageUseCase);
	}

	createImagesListedStore(initialData?: readonly ImageInfoViewModel[]): ImagesListedStore {
		return new ImagesListedStore(this.useCases.listImagesUseCase, initialData);
	}

	createImageLoadedStore(initialData?: ImageInfoViewModel): ImageLoadedStore {
		return new ImageLoadedStore(this.useCases.getImageInfoUseCase, initialData);
	}

	createPostsListedStore(initialData?: readonly PostInfoViewModel[]): PostsListedStore {
		return new PostsListedStore(this.useCases.getAllPostsUseCase, initialData);
	}

	createPostLoadedStore(initialData?: PostViewModel): PostLoadedStore {
		return new PostLoadedStore(this.useCases.getPostUseCase, initialData);
	}

	createPostCreatedStore(): PostCreatedStore {
		return new PostCreatedStore(this.useCases.createPostUseCase);
	}

	createPostUpdatedStore(): PostUpdatedStore {
		return new PostUpdatedStore(this.useCases.updatePostUseCase);
	}

	createLabelsListedStore(initialData?: readonly LabelViewModel[]): LabelsListedStore {
		return new LabelsListedStore(this.useCases.getAllLabelsUseCase, initialData);
	}

	createLabelLoadedStore(initialData?: LabelViewModel): LabelLoadedStore {
		return new LabelLoadedStore(this.useCases.getLabelUseCase, initialData);
	}

	createLabelCreatedStore(): LabelCreatedStore {
		return new LabelCreatedStore(this.useCases.createLabelUseCase);
	}

	createLabelUpdatedStore(): LabelUpdatedStore {
		return new LabelUpdatedStore(this.useCases.updateLabelUseCase);
	}
}

class ApiServices {
	private fetchFn: typeof fetch;

	private _authApiService?: AuthApiService;
	private _imageApiService?: ImageApiService;
	private _postApiService?: PostApiService;
	private _labelApiService?: LabelApiService;

	constructor(fetchFn: typeof fetch) {
		this.fetchFn = fetchFn;
	}

	get authApiService(): AuthApiService {
		this._authApiService ??= new AuthApiServiceImpl(this.fetchFn);
		return this._authApiService;
	}

	get imageApiService(): ImageApiService {
		this._imageApiService ??= new ImageApiServiceImpl(this.fetchFn);
		return this._imageApiService;
	}

	get postApiService(): PostApiService {
		this._postApiService ??= new PostApiServiceImpl(this.fetchFn);
		return this._postApiService;
	}

	get labelApiService(): LabelApiService {
		this._labelApiService ??= new LabelApiServiceImpl(this.fetchFn);
		return this._labelApiService;
	}
}

class Repositories {
	private apiServices: ApiServices;

	private _authRepository?: AuthRepository;
	private _imageRepository?: ImageRepository;
	private _postRepository?: PostRepository;
	private _labelRepository?: LabelRepository;

	constructor(apiServices: ApiServices) {
		this.apiServices = apiServices;
	}

	get authRepository(): AuthRepository {
		this._authRepository ??= new AuthRepositoryImpl(this.apiServices.authApiService);
		return this._authRepository;
	}

	get imageRepository(): ImageRepository {
		this._imageRepository ??= new ImageRepositoryImpl(this.apiServices.imageApiService);
		return this._imageRepository;
	}

	get postRepository(): PostRepository {
		this._postRepository ??= new PostRepositoryImpl(this.apiServices.postApiService);
		return this._postRepository;
	}

	get labelRepository(): LabelRepository {
		this._labelRepository ??= new LabelRepositoryImpl(this.apiServices.labelApiService);
		return this._labelRepository;
	}
}

class UseCases {
	private repositories: Repositories;

	private _getCurrentUserUseCase?: GetCurrentUserUseCase;
	private _uploadImageUseCase?: UploadImageUseCase;
	private _listImagesUseCase?: ListImagesUseCase;
	private _getImageInfoUseCase?: GetImageInfoUseCase;
	private _getAllPostsUseCase?: GetAllPostsUseCase;
	private _getPostUseCase?: GetPostUseCase;
	private _createPostUseCase?: CreatePostUseCase;
	private _updatePostUseCase?: UpdatePostUseCase;
	private _getAllLabelsUseCase?: GetAllLabelsUseCase;
	private _getLabelUseCase?: GetLabelUseCase;
	private _createLabelUseCase?: CreateLabelUseCase;
	private _updateLabelUseCase?: UpdateLabelUseCase;

	constructor(repositories: Repositories) {
		this.repositories = repositories;
	}

	get getCurrentUserUseCase(): GetCurrentUserUseCase {
		this._getCurrentUserUseCase ??= new GetCurrentUserUseCase(this.repositories.authRepository);
		return this._getCurrentUserUseCase;
	}

	get uploadImageUseCase(): UploadImageUseCase {
		this._uploadImageUseCase ??= new UploadImageUseCase(this.repositories.imageRepository);
		return this._uploadImageUseCase;
	}

	get listImagesUseCase(): ListImagesUseCase {
		this._listImagesUseCase ??= new ListImagesUseCase(this.repositories.imageRepository);
		return this._listImagesUseCase;
	}

	get getImageInfoUseCase(): GetImageInfoUseCase {
		this._getImageInfoUseCase ??= new GetImageInfoUseCase(this.repositories.imageRepository);
		return this._getImageInfoUseCase;
	}

	get getAllPostsUseCase(): GetAllPostsUseCase {
		this._getAllPostsUseCase ??= new GetAllPostsUseCase(this.repositories.postRepository);
		return this._getAllPostsUseCase;
	}

	get getPostUseCase(): GetPostUseCase {
		this._getPostUseCase ??= new GetPostUseCase(this.repositories.postRepository);
		return this._getPostUseCase;
	}

	get createPostUseCase(): CreatePostUseCase {
		this._createPostUseCase ??= new CreatePostUseCase(this.repositories.postRepository);
		return this._createPostUseCase;
	}

	get updatePostUseCase(): UpdatePostUseCase {
		this._updatePostUseCase ??= new UpdatePostUseCase(this.repositories.postRepository);
		return this._updatePostUseCase;
	}

	get getAllLabelsUseCase(): GetAllLabelsUseCase {
		this._getAllLabelsUseCase ??= new GetAllLabelsUseCase(this.repositories.labelRepository);
		return this._getAllLabelsUseCase;
	}

	get getLabelUseCase(): GetLabelUseCase {
		this._getLabelUseCase ??= new GetLabelUseCase(this.repositories.labelRepository);
		return this._getLabelUseCase;
	}

	get createLabelUseCase(): CreateLabelUseCase {
		this._createLabelUseCase ??= new CreateLabelUseCase(this.repositories.labelRepository);
		return this._createLabelUseCase;
	}

	get updateLabelUseCase(): UpdateLabelUseCase {
		this._updateLabelUseCase ??= new UpdateLabelUseCase(this.repositories.labelRepository);
		return this._updateLabelUseCase;
	}
}
