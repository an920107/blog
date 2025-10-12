export enum StatusType {
	Idle,
	Loading,
	Success,
	Error,
}

export interface IdleState<T> {
	status: StatusType.Idle;
	data?: T;
}

export interface LoadingState<T> {
	status: StatusType.Loading;
	data?: T;
}

export interface SuccessState<T> {
	status: StatusType.Success;
	data: T;
}

export interface ErrorState<T> {
	status: StatusType.Error;
	data?: T;
	error: Error;
}

export type AsyncState<T> = IdleState<T> | LoadingState<T> | SuccessState<T> | ErrorState<T>;
