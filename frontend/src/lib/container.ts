import type { AuthApiService } from '$lib/auth/adapter/gateway/authApiService';
import { AuthRepositoryImpl } from '$lib/auth/adapter/gateway/authRepositoryImpl';
import { AuthLoadedStore } from '$lib/auth/adapter/presenter/authLoadedStore';
import type { AuthRepository } from '$lib/auth/application/gateway/authRepository';
import { GetCurrentUserUseCase } from '$lib/auth/application/useCase/getCurrentUserUseCase';
import { AuthApiServiceImpl } from '$lib/auth/framework/api/authApiServiceImpl';
import type { LabelApiService } from '$lib/label/adapter/gateway/labelApiService';
import { LabelRepositoryImpl } from '$lib/label/adapter/gateway/labelRepositoryImpl';
import { LabelCreatedStore } from '$lib/label/adapter/presenter/labelCreatedStore';
import type { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
import { LabelsListedStore } from '$lib/label/adapter/presenter/labelsListedStore';
import type { LabelRepository } from '$lib/label/application/gateway/labelRepository';
import { CreateLabelUseCase } from '$lib/label/application/useCase/createLabelUseCase';
import { GetAllLabelsUseCase } from '$lib/label/application/useCase/getAllLabelsUseCase';
import { LabelApiServiceImpl } from '$lib/label/framework/api/labelApiServiceImpl';
import type { ImageApiService } from '$lib/image/adapter/gateway/imageApiService';
import { ImageRepositoryImpl } from '$lib/image/adapter/gateway/imageRepositoryImpl';
import { ImageUploadedStore } from '$lib/image/adapter/presenter/imageUploadedStore';
import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';
import { UploadImageUseCase } from '$lib/image/application/useCase/uploadImageUseCase';
import { ImageApiServiceImpl } from '$lib/image/framework/api/imageApiServiceImpl';
import type { PostApiService } from '$lib/post/adapter/gateway/postApiService';
import { PostRepositoryImpl } from '$lib/post/adapter/gateway/postRepositoryImpl';
import { PostCreatedStore } from '$lib/post/adapter/presenter/postCreatedStore';
import type { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';
import { PostsListedStore } from '$lib/post/adapter/presenter/postsListedStore';
import { PostLoadedStore } from '$lib/post/adapter/presenter/postLoadedStore';
import type { PostViewModel } from '$lib/post/adapter/presenter/postViewModel';
import type { PostRepository } from '$lib/post/application/gateway/postRepository';
import { CreatePostUseCase } from '$lib/post/application/useCase/createPostUseCase';
import { GetAllPostsUseCase } from '$lib/post/application/useCase/getAllPostsUseCase';
import { GetPostUseCase } from '$lib/post/application/useCase/getPostUseCase';
import { PostApiServiceImpl } from '$lib/post/framework/api/postApiServiceImpl';

export class Container {
	private useCases: UseCases;

	constructor(fetchFn: typeof fetch) {
		const apiServices = new ApiServices(fetchFn);
		const repositories = new Repositories(apiServices);
		this.useCases = new UseCases(repositories);
	}

	createAuthLoadedStore(): AuthLoadedStore {
		return new AuthLoadedStore(this.useCases.getCurrentUserUseCase);
	}

	createImageUploadedStore(): ImageUploadedStore {
		return new ImageUploadedStore(this.useCases.uploadImageUseCase);
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

	createLabelsListedStore(initialData?: readonly LabelViewModel[]): LabelsListedStore {
		return new LabelsListedStore(this.useCases.getAllLabelsUseCase, initialData);
	}

	createLabelCreatedStore(): LabelCreatedStore {
		return new LabelCreatedStore(this.useCases.createLabelUseCase);
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
	private _getAllPostsUseCase?: GetAllPostsUseCase;
	private _getPostUseCase?: GetPostUseCase;
	private _createPostUseCase?: CreatePostUseCase;
	private _getAllLabelsUseCase?: GetAllLabelsUseCase;
	private _createLabelUseCase?: CreateLabelUseCase;

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

	get getAllLabelsUseCase(): GetAllLabelsUseCase {
		this._getAllLabelsUseCase ??= new GetAllLabelsUseCase(this.repositories.labelRepository);
		return this._getAllLabelsUseCase;
	}

	get createLabelUseCase(): CreateLabelUseCase {
		this._createLabelUseCase ??= new CreateLabelUseCase(this.repositories.labelRepository);
		return this._createLabelUseCase;
	}
}
