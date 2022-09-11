import { invoke } from '@tauri-apps/api';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

enum TauriCommands {
  GetImages = "get_images",
  GetDeviceInformation = "get_device_information",
};

export type Building = {
  id: number;
  name: string;
  color: string;
}

export type Floor = {
  id: number;
  name: string;
  building: {
    id: number;
    name: string;
    color: string;
  };
  devices: {
    id: number;
    name: string;
    description: string;
  }[];
};

const getImages = async () => {
  try {
    const images: string[] = await invoke(TauriCommands.GetImages);

    return images;
  } catch (err) {
    throw err;
  }
};

export type DeviceInformation = {
  id: number;
  name: string;
  description: string;
  location: string;
};

export const getDeviceInformation = async (): Promise<DeviceInformation> => {
  try {
    const deviceInformation: DeviceInformation = await invoke(TauriCommands.GetDeviceInformation);
    return deviceInformation;
  } catch (err) {
    throw err;
  }
}

export const subscribeToAnnouncementUpdates = async (callback: () => void): Promise<UnlistenFn> => {
  try {
    const unlisten = await listen('listen_media_update', callback);

    return unlisten;
  } catch (err) {
    throw err;
  }
}

export const tauri = {
  getImages,
  getDeviceInformation,
}
