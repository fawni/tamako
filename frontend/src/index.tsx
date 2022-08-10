/* @refresh reload */
import { render } from "solid-js/web";
import "tailwindcss/tailwind.css";

import "./index.scss";
import App from "./App";

render(() => <App />, document.getElementById("root") as HTMLElement);
