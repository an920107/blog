export enum StatusType {
	Idle,
	Loading,
	Success,
	Error
}

export interface IdleState {
	status: StatusType.Idle;
}

export interface LoadingState {
	status: StatusType.Loading;
}

export interface SuccessState<T> {
	status: StatusType.Success;
	data: T;
}

export interface ErrorState {
	status: StatusType.Error;
	error: Error;
}

export type AsyncState<T> = IdleState | LoadingState | SuccessState<T> | ErrorState;
