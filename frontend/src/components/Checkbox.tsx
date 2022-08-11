import type { Component } from "solid-js";

import styles from "./Checkbox.module.scss";

const Checkbox: Component = () => {
	return (
		<label class={styles.container}>
			private?
			<input type="checkbox"></input>
			<span class={styles.checkmark}></span>
		</label>
	);
};

export default Checkbox;
