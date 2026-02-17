const isTauri = typeof window !== 'undefined' && '__TAURI__' in window;

export async function invoke<T>(
  command: string,
  payload?: Record<string, unknown>
): Promise<T> {
  if (command.trim() === '') {
    throw new Error('command must not be empty');
  }

  if (isTauri) {
    const { invoke: tauriInvoke } = await import('@tauri-apps/api/core');
    return tauriInvoke<T>(command, payload);
  }

  return Promise.resolve({ command, payload } as unknown as T);
}
