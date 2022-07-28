import React, { useContext } from 'react';
import { MenuContext, MenuOptions } from './constants';

const DeviceProfile = () => {
  const { deviceInformation, setActiveMenu } = useContext(MenuContext);

  return (
    <div>
      <h2>Device Information</h2>

      {deviceInformation !== null ? (
          <>
            <p>Device ID: {deviceInformation.id}</p>
            <p>Device Name: {deviceInformation.name}</p>
            <p>Device Location: {deviceInformation.location}</p>
            <p>Device Description: {deviceInformation.description}</p>
          </>
      ) : (<p>Device has not been registered in the system</p>)}

      <button onClick={() => setActiveMenu(MenuOptions.MainMenu)}>
        <p>Back</p>
      </button>
    </div>
  )
};

export default DeviceProfile;
