import { invoke } from '@tauri-apps/api';

enum TauriCommands {
  GetImages = "get_images",
  GetBuildings = "get_buildings",
  GetFloors = "get_floors",
  CreateDevice = "create_device",
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

export const getBuildings = async (): Promise<Building[]> => {
  try {
    const contents: Building[] = await invoke(TauriCommands.GetBuildings);

    return contents;
  } catch (err) {
    throw err;
  }
};

export const getFloors = async (buildingId: number): Promise<Floor[]> => {
  try {
    const contents: Floor[] = await invoke(TauriCommands.GetFloors, { buildingId });

    return contents;
  } catch (err) {
    throw err;
  }
};

export type CreateDeviceParams = {
  name: string;
  description: string;
  floorId: number;
  isLinked: boolean;
};

export const createDevice = async (body: CreateDeviceParams) => {
  try {
    await invoke(TauriCommands.CreateDevice, { body });
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

export const tauri = {
  getImages,
  getBuildings,
  getFloors,
  createDevice,
  getDeviceInformation,
}
