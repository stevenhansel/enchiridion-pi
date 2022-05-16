const invoke = window.__TAURI__.invoke

export async function getImages() {
  return await invoke("get_images");
}
