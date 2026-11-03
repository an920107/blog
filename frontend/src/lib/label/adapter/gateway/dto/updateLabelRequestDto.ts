import { ColorDto } from '$lib/label/adapter/gateway/dto/colorDto';
import type { UpdateLabelParams } from '$lib/label/application/port/labelRepository';

export class UpdateLabelRequestDto {
	readonly name: string;
	readonly color: ColorDto;

	private constructor(props: { name: string; color: ColorDto }) {
		this.name = props.name;
		this.color = props.color;
	}

	static fromParams(params: UpdateLabelParams): UpdateLabelRequestDto {
		return new UpdateLabelRequestDto({
			name: params.name,
			color: ColorDto.fromEntity(params.color),
		});
	}

	toJson() {
		return {
			name: this.name,
			color: this.color.toJson(),
		};
	}
}
