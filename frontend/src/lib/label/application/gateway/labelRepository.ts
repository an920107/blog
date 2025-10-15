import type { Color } from '$lib/label/domain/entity/color';
import type { Label } from '$lib/label/domain/entity/label';

export interface LabelRepository {
	getAllLabels(): Promise<Label[]>;
	createLabel(params: CreateLabelParams): Promise<Label>;
}

export interface CreateLabelParams {
	name: string;
	color: Color;
}
