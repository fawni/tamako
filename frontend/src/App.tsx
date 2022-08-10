import type { Component } from "solid-js";

import styles from "./App.module.scss";

async function sendWhispers() {
	try {
	} catch (err) {
		console.log(err);
	}
	//   console.log("Whispering...");
}

const App: Component = () => {
	return (
		// <div class="bg-primary min-h-screen text-center font-iosevka selection:bg-accent selection:text-white box-border flex-nowrap">
		<div class={styles.App}>
			<div class="flex flex-col  items-center justify-center ">
				<h1>
					<span class={styles.accent}>~</span>tamako
				</h1>
				<form>
					<input
						type="text"
						maxlength="600"
						placeholder="whisper me sommething!"
					></input>
					<button
						onclick={() => {
							alert("todo");
						}}
					>
						send
					</button>
				</form>
			</div>
		</div>
	);
};

export default App;
