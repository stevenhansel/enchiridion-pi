import React, { useState, useEffect } from 'react';
import './Menu.css';

import MainMenu from './MainMenu';
import DeviceProfile from './DeviceProfile';
import Registration from './Registration';
import { MenuContext, MenuOptions } from './constants';
import { tauri, DeviceInformation } from '../tauri';

type Props = {
  close: () => void;
};

const Menu = ({ close }: Props) => {
  const [activeMenu, setActiveMenu] = useState<MenuOptions>(MenuOptions.MainMenu);
  const [deviceInformation, setDeviceInformation] = useState<DeviceInformation | null>(null);

  const render = (): JSX.Element => {
    switch (activeMenu) {
      case MenuOptions.MainMenu:
        return <MainMenu />;
      case MenuOptions.DeviceProfile:
        return <DeviceProfile />
      case MenuOptions.Registration:
        return <Registration />
      default:
        return <MainMenu />
    }
  }

  useEffect(() => {
    tauri.getDeviceInformation()
      .then((info) => setDeviceInformation(info));
  }, [])

  return (
    <MenuContext.Provider value={{
      activeMenu,
      setActiveMenu,
      deviceInformation,
      setDeviceInformation,
      close,
    }}>
      <div className="backdrop">
        <div className="container">
          {render()}
        </div>
      </div>
    </MenuContext.Provider>
  );
};

export default Menu;
