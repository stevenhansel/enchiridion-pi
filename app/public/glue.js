const invoke = window.__TAURI__.invoke

export async function invokePing(message) {
  return await invoke("ping", { message });
}
