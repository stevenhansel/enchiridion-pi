import { createContext } from "react";

export enum MenuOptions {
  MainMenu,
  DeviceInformation,
  Registration,
  CloseMenu,
};

export type SetState<T> = React.Dispatch<React.SetStateAction<T>>;

export type MenuContextType = {
  activeMenu: MenuOptions,
  setActiveMenu: SetState<MenuOptions>,
  close: () => void,
}
export const MenuContext = createContext<MenuContextType>({
  activeMenu: MenuOptions.MainMenu,
  setActiveMenu: () => {},
  close: () => {},
});

export type Building = {};

export type Floor = {
  id: number;
  name: string;
  building: {
    id: number;
    name: string;
    color: string;
  };
}
