import { invoke } from "@tauri-apps/api/core";

let clipboardHistory: string[] = [];
const list = document.getElementById("clipboard-list");

async function updateClipboard() {
  const text: string = await invoke("get_clipboard");
  if (text && clipboardHistory[0] !== text) {
    clipboardHistory.unshift(text);
    if (clipboardHistory.length > 20) clipboardHistory.pop();
    console.log(text);
    renderList();
  }
}
function renderList() {
  if (!list) return;
  list.innerHTML = "";
  const maxLength = 50; // max characters to show

  clipboardHistory.forEach((entry) => {
    const div = document.createElement("div");
    div.className = "entry";

    // truncate long text
    const displayText = entry.length > maxLength ? entry.slice(0, maxLength) + "..." : entry;
    div.textContent = displayText;

    div.title = entry; // show full text on hover
    div.onclick = async () => {
      await invoke("write_clipboard", { text: entry });
      alert("Copied back to clipboard!");
    };
    list.appendChild(div);
  });
}

// Poll clipboard every 500ms
setInterval(updateClipboard, 500);
// Disable right-click context menu
window.addEventListener("contextmenu", (e) => {
  e.preventDefault();
});

import { register } from "@tauri-apps/plugin-global-shortcut";
window.addEventListener("DOMContentLoaded", async () => {
  await new Promise<void>((resolve) =>
    window.addEventListener("tauri://ready", () => resolve())
  );

  await register("CommandOrControl+Alt+U", () => {
    console.log("Shortcut triggered!");
    alert("Pressed shortcut!");
  });

  console.log("Global shortcut registered!");
});