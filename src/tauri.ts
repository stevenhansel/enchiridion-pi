import { invoke } from '@tauri-apps/api';

enum TauriCommands {
  GetImages = "get_images",
  GetBuildings = "get_buildings",
  GetFloors = "get_floors",
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

export const tauri = {
  getImages,
  getBuildings,
  getFloors,
}
