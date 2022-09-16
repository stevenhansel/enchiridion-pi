import { invoke } from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { ApplicationErrorCode } from "./constants";

enum TauriCommands {
  GetImages = "get_images",
  GetDeviceInformation = "get_device_information",
  Link = "link",
  Unlink = "unlink",
}

enum TauriEvents {
  MediaUpdate = "media_update",
}

export type TauriErrorObject = {
  errorCode: ApplicationErrorCode;
  messages: string[];
};

export const isTauriErrorObject = <T>(
  response: TauriCommandResponse<T>
): boolean => {
  return typeof response === "object" && "errorCode" in response;
};

export type TauriCommandResponse<T> = T | TauriErrorObject;

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

const getImages = async () => {
  try {
    const images: string[] = await invoke(TauriCommands.GetImages);

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

export const subscribeToAnnouncementUpdates = async (
  callback: () => void
): Promise<UnlistenFn> => {
  try {
    const unlisten = await listen(TauriEvents.MediaUpdate, callback);

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

export const tauri = {
  getImages,
  getDeviceInformation,
  link,
  unlink,
};
