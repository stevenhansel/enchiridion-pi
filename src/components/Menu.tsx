import React, { useState } from 'react';
import './Menu.css';

import MainMenu from './MainMenu';
import DeviceInformation from './DeviceInformation';
import Registration from './Registration';
import { MenuContext, MenuOptions } from './constants';

type Props = {
  close: () => void;
};

const Menu = ({ close }: Props) => {
  const [activeMenu, setActiveMenu] = useState<MenuOptions>(MenuOptions.MainMenu);

  const render = (): JSX.Element => {
    switch (activeMenu) {
      case MenuOptions.MainMenu:
        return <MainMenu />;
      case MenuOptions.DeviceInformation:
        return <DeviceInformation />
      case MenuOptions.Registration:
        return <Registration />
      default:
        return <MainMenu />
    }
  }

  return (
    <MenuContext.Provider value={{
      activeMenu,
      setActiveMenu,
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
