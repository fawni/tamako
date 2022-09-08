import { ParentComponent, ParentProps } from "solid-js";

import "./Checkbox.scss";

const Checkbox: ParentComponent = (props: ParentProps) => {
	return (
		<label class="checkbox-container">
			{props.children}
			<input type="checkbox" id="whisper-private"></input>
			<span class="checkmark"></span>
		</label>
	);
};

export default Checkbox;
