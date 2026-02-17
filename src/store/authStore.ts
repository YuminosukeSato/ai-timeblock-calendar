import { create } from 'zustand';

interface AuthStore {
  connected: boolean;
  setConnected: (connected: boolean) => void;
}

export const useAuthStore = create<AuthStore>((set) => ({
  connected: false,
  setConnected: (connected) => set({ connected })
}));
