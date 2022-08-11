import type { Component } from "solid-js";

import styles from "./App.module.scss";
import Checkbox from "./components/checkbox";

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

				<div class={styles.whisper}>
					<input
						class={styles.text}
						type="text"
						maxlength="20"
						placeholder="anonymous"
					></input>

					<input
						class={styles.text}
						type="text"
						maxlength="100"
						placeholder="( =ω=)..nyaa"
					></input>

					<Checkbox></Checkbox>
					{/* <label class={styles.container}>
						private?
						<input type="checkbox"></input>
						<span class={styles.checkmark}></span>
					</label> */}

					{/* <div class={styles.checkbox}>
						<input type="checkbox" id="private" name="private"></input>
						<label for="private">private?</label>
					</div> */}
				</div>

				<div>
					<button
						onclick={() => {
							alert("todo");
						}}
					>
						whisper
					</button>
				</div>
				{/* <div>
					<br></br>
					<ul>
						<li>
							love you bro{" "}
							<span class={styles.timestamp}>25 Dec 2022, 05:42:24 PM</span>
						</li>
						<li>
							<span class={styles.accent}>name:</span>
							you kinda suck{" "}
							<span class={styles.timestamp}>16 Aug 2022, 08:28:16 PM</span>
						</li>
						<li>
							this is so true yeah lmao hiii{" "}
							<span class={styles.timestamp}>16 Aug 2022, 08:28:16 PM</span>
						</li>
					</ul>
				</div> */}
			</div>
		</div>
	);
};

export default App;
