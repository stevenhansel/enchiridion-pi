import React, { useContext } from 'react';
import './MainMenu.css';

import { MenuContext, MenuOptions } from './constants';

const MainMenu = () => {
  const { close, setActiveMenu } = useContext(MenuContext);

  return (
    <div>
      <div className="menu-item" onClick={() => setActiveMenu(MenuOptions.DeviceInformation)}>
        <p>Device Information</p>
      </div>

      <div className="menu-item" onClick={() => setActiveMenu(MenuOptions.Registration)}>
        <p>Registration</p>
      </div>

      <div className="menu-item" onClick={() => close()}>
        <p>Close</p>
      </div>
    </div>
  );
};

export default MainMenu;
