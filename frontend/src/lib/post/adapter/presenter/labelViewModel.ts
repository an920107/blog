import { ColorViewModel } from '$lib/post/adapter/presenter/colorViewModel';
import type { Label } from '$lib/post/domain/entity/label';

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
			color: ColorViewModel.fromEntity(label.color)
		});
	}
}
