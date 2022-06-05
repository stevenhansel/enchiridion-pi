const invoke = window.__TAURI__.invoke
const event = window.__TAURI__.event;

export async function getImages() {
  return await invoke("get_images");
}

export async function listenMediaUpdate(callback) {
  return await event.listen("test-ipc", callback);
}
