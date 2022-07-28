import React, { useContext } from 'react';
import { MenuContext, MenuOptions } from './constants';

const DeviceInformation = () => {
  const { setActiveMenu } = useContext(MenuContext);

  return (
    <div>
      <h2>Device Information</h2>

      <div onClick={() => setActiveMenu(MenuOptions.MainMenu)}>
        <p>Back</p>
      </div>
    </div>
  )
};

export default DeviceInformation;
