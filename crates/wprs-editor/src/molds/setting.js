import { props } from "wprs";

export const setting = (value, { params }) => props.get(params.prop) ?? 0;
