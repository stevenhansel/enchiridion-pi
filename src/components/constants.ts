import { createContext } from "react";
import { DeviceInformation } from '../tauri';

export enum MenuOptions {
  MainMenu,
  DeviceProfile,
  Registration,
  CloseMenu,
};

export type SetState<T> = React.Dispatch<React.SetStateAction<T>>;

export type MenuContextType = {
  activeMenu: MenuOptions,
  setActiveMenu: SetState<MenuOptions>,
  deviceInformation: DeviceInformation | null,
  setDeviceInformation: SetState<DeviceInformation | null>,
  close: () => void,
}
export const MenuContext = createContext<MenuContextType>({
  activeMenu: MenuOptions.MainMenu,
  setActiveMenu: () => {},
  deviceInformation: null,
  setDeviceInformation: () => {},
  close: () => {},
});
