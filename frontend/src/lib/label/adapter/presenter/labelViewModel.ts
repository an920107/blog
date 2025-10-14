import {
	ColorViewModel,
	type DehydratedColorProps,
} from '$lib/label/adapter/presenter/colorViewModel';
import type { Label } from '$lib/label/domain/entity/label';

export class LabelViewModel {
	readonly id: number;
	readonly name: string;
	readonly color: ColorViewModel;

	private constructor(props: { id: number; name: string; color: ColorViewModel }) {
		this.id = props.id;
		this.name = props.name;
		this.color = props.color;
	}

	static fromEntity(label: Label): LabelViewModel {
		return new LabelViewModel({
			id: label.id,
			name: label.name,
			color: ColorViewModel.fromEntity(label.color),
		});
	}

	static rehydrate(props: DehydratedLabelProps): LabelViewModel {
		return new LabelViewModel({
			id: props.id,
			name: props.name,
			color: ColorViewModel.rehydrate(props.color),
		});
	}

	dehydrate(): DehydratedLabelProps {
		return {
			id: this.id,
			name: this.name,
			color: this.color.dehydrate(),
		};
	}
}

export interface DehydratedLabelProps {
	id: number;
	name: string;
	color: DehydratedColorProps;
}
