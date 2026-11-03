enum AsyncStateStatus {
	Idle = 'idle',
	Loading = 'loading',
	Success = 'success',
	Error = 'error',
}

export abstract class AsyncState<T> {
	abstract readonly status: AsyncStateStatus;
	abstract readonly data: T | null;
	abstract readonly error: Error | null;

	static idle<T>(data: T | null): IdleState<T> {
		return new IdleState(data);
	}

	static loading<T>(data: T | null): LoadingState<T> {
		return new LoadingState(data);
	}

	static success<T>(data: T): SuccessState<T> {
		return new SuccessState(data);
	}

	static error<T>(error: unknown, data: T | null): ErrorState<T> {
		const errorInstance = error instanceof Error ? error : new Error(String(error));
		return new ErrorState(errorInstance, data);
	}

	isIdle(): this is IdleState<T> {
		return this.status === AsyncStateStatus.Idle;
	}

	isLoading(): this is LoadingState<T> {
		return this.status === AsyncStateStatus.Loading;
	}

	isSuccess(): this is SuccessState<T> {
		return this.status === AsyncStateStatus.Success;
	}

	isError(): this is ErrorState<T> {
		return this.status === AsyncStateStatus.Error;
	}
}

class IdleState<T> extends AsyncState<T> {
	readonly status = AsyncStateStatus.Idle;
	readonly data: T | null;
	readonly error = null;

	constructor(data: T | null) {
		super();
		this.data = data;
	}

	toLoading(): LoadingState<T> {
		return new LoadingState(this.data);
	}
}

class LoadingState<T> extends AsyncState<T> {
	readonly status = AsyncStateStatus.Loading;
	readonly data: T | null;
	readonly error = null;

	constructor(data: T | null) {
		super();
		this.data = data;
	}

	toSuccess(data: T): SuccessState<T> {
		return new SuccessState(data);
	}

	toError(error: Error): ErrorState<T> {
		return new ErrorState(error, this.data);
	}
}

class SuccessState<T> extends AsyncState<T> {
	readonly status = AsyncStateStatus.Success;
	readonly data: T;
	readonly error = null;

	constructor(data: T) {
		super();
		this.data = data;
	}

	toLoading(): LoadingState<T> {
		return new LoadingState(this.data);
	}
}

class ErrorState<T> extends AsyncState<T> {
	readonly status = AsyncStateStatus.Error;
	readonly data: T | null;
	readonly error: Error;

	constructor(error: Error, data: T | null) {
		super();
		this.error = error;
		this.data = data;
	}

	toLoading(): LoadingState<T> {
		return new LoadingState(this.data);
	}
}
