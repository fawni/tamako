import { Component, createSignal, onMount, For, Show } from "solid-js";
import toast, { Toaster } from "solid-toast";

import "./App.scss";
import Checkbox from "./components/checkbox";

async function sendWhispers() {
	const nameEl = document.getElementById("whisper-name");
	const textEl = document.getElementById("whisper-text");
	const privateEl = document.getElementById("whisper-private");

	let data = {
		name: nameEl?.value,
		text: textEl?.value,
		private: privateEl?.checked,
	};

	let res = await fetch("http://localhost:3030/api/whisper", {
		method: "POST",
		headers: {
			"content-type": "application/json",
		},
		body: JSON.stringify(data, (_, v) => (v === "" ? undefined : v)),
	});

	if (res.status === 201) {
		toast.success("sent! >w<");
	} else {
		toast.error("an error occurred TwT");
	}
}

const App: Component = () => {
	const [whispers, setWhispers] = createSignal([]);

	onMount(async () => {
		const res = await fetch("http://localhost:3030/api/whispers");
		setWhispers(await res.json());
	});

	return (
		<div class="App">
			<div>
				<h1>
					<span class="accent">~</span>tamako
				</h1>

				<div class="whisper-container">
					<input
						class="input-text"
						type="text"
						id="whisper-name"
						maxlength="20"
						placeholder="anonymous"
					></input>

					<input
						class="input-text"
						type="text"
						id="whisper-text"
						maxlength="100"
						placeholder="( =ω=)..nyaa"
					></input>

					<Checkbox>private?</Checkbox>
				</div>

				<div>
					<button onclick={sendWhispers}>whisper</button>
					<Toaster
						toastOptions={{
							style: {
								background: "#2a2331",
								color: "#dedbeb",
							},
						}}
					/>
				</div>

				<div class="whispers">
					<br></br>
					<ul>
						<For each={whispers()}>
							{(whisper) => (
								<li>
									<Show when={whisper.name}>
										<span class="accent">{whisper.name}:</span>
									</Show>
									{whisper.text}
									<span class="timestamp"> • {whisper.timestamp}</span>
								</li>
							)}
						</For>
					</ul>
				</div>
			</div>
		</div>
	);
};

export default App;
