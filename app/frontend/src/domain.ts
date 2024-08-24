export interface Ascent {
	type: 'f' | 'os' | 'rp';
	zlaggableName: string;
	countryName: String;
	cragName: string;
	areaName?: string;
	difficulty: string;
	comment?: string;
	date: Date;
}
