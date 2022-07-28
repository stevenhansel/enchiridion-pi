import React, { useContext } from 'react';
import './MainMenu.css';

import { MenuContext, MenuOptions } from './constants';

const MainMenu = () => {
  const { deviceInformation, close, setActiveMenu } = useContext(MenuContext);

  return (
    <div>
      {deviceInformation !== null ? (
        <div className="menu-item" onClick={() => setActiveMenu(MenuOptions.DeviceProfile)}>
          <p>Profile</p>
        </div>
      ) : (
        <div className="menu-item" onClick={() => setActiveMenu(MenuOptions.Registration)}>
          <p>Registration</p>
        </div>
      )}


      <div className="menu-item" onClick={() => close()}>
        <p>Close</p>
      </div>
    </div>
  );
};

export default MainMenu;
