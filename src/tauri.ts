import { invoke } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { ApplicationErrorCode } from "./constants";

enum TauriCommands {
  GetAnnouncements = "get_announcements",
  GetDeviceInformation = "get_device_information",
  Link = "link",
  Unlink = "unlink",
  IsNetworkConnected = "is_network_connected",
  IsCameraEnabled = "is_camera_enabled",
  SpawnStatusPoller = "spawn_status_poller",
  SpawnCamera = "spawn_camera",
  SpawnAnnouncementConsumer = "spawn_announcement_consumer",
}

enum TauriEvents {
  MediaUpdateStart = "media_update_start",
  MediaUpdateEnd = "media_update_end",
}

export type TauriErrorObject = {
  errorCode: ApplicationErrorCode;
  messages: string[];
};

export const isTauriErrorObject = <T>(
  response: TauriCommandResponse<T>
): boolean => {
  return typeof response === "object" && response !== null && "errorCode" in response;
};

export type TauriCommandResponse<T> = T | TauriErrorObject;

export type Announcement = {
  id: number;
  announcement_id: number;
  local_path: string;
  media_type: string;
  media_duration: number | null;
}

export type DeviceInformation = {
  id: number;
  name: string;
  description: string;
  location: string;
  floorId: number;
  buildingId: number;
  createdAt: string;
  updatedAt: string;
};

const getAnnouncements = async () => {
  try {
    const images: Announcement[] = await invoke(TauriCommands.GetAnnouncements);

    return images;
  } catch (err) {
    throw err;
  }
};

export const getDeviceInformation = async (): Promise<DeviceInformation> => {
  try {
    const deviceInformation: DeviceInformation = await invoke(
      TauriCommands.GetDeviceInformation
    );
    return deviceInformation;
  } catch (err) {
    throw err;
  }
};

export const listenToMediaUpdateStart = async (
  callback: () => void
): Promise<UnlistenFn> => {
  try {
    const unlisten = await listen(TauriEvents.MediaUpdateStart, callback);

    return unlisten;
  } catch (err) {
    throw err;
  }
};

export const listenToMediaUpdateEnd = async (
  callback: () => void
): Promise<UnlistenFn> => {
  try {
    const unlisten = await listen(TauriEvents.MediaUpdateEnd, callback);

    return unlisten;
  } catch (err) {
    throw err;
  }
};

export const link = async (
  accessKeyId: string,
  secretAccessKey: string
): Promise<TauriCommandResponse<DeviceInformation>> => {
  try {
    const device = await invoke(TauriCommands.Link, {
      accessKeyId,
      secretAccessKey,
    });

    return device as DeviceInformation;
  } catch (err) {
    return err as TauriErrorObject;
  }
};

export const unlink = async (): Promise<TauriCommandResponse<void>> => {
  try {
    await invoke(TauriCommands.Unlink);
  } catch (err) {
    return err as TauriErrorObject;
  }
};

export const isNetworkConnected = async () => {
  const isNetworkConnected = await invoke(TauriCommands.IsNetworkConnected);
  return isNetworkConnected as boolean;
};

export const isCameraEnabled = async () => {
  return (await invoke(TauriCommands.IsCameraEnabled)) as boolean;
};

export const spawnStatusPoller = async () => {
  await invoke(TauriCommands.SpawnStatusPoller);
};

export const spawnCamera = async () => {
  await invoke(TauriCommands.SpawnCamera);
};

export const spawnAnnouncementConsumer = async () => {
  await invoke(TauriCommands.SpawnAnnouncementConsumer);
};

export const tauri = {
  getAnnouncements,
  getDeviceInformation,
  link,
  unlink,
  isNetworkConnected,
  isCameraEnabled,
  spawnCamera,
  spawnAnnouncementConsumer,
};
