import React, { useContext, useState } from 'react';
import { Building, Floor, MenuContext, MenuOptions } from './constants';

const Registration = () => {
  const { setActiveMenu } = useContext(MenuContext);

  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [floorId, setFloorId] = useState<number | null>(null);

  const [buildings, setBuildings] = useState<Building[]>([]);
  const [floors, setFloors] = useState<Floor[]>([]);

  return (
    <div>
      <h2>Device Registration</h2>
      <p>In order to register this device into Enchiridion System, please fill in the device information in the form below:</p>

      <div>
        <p>Device Name</p>
        <input value={name} onChange={(e) => setName(e.target.value)} />
      </div>

      <div>
        <p>Device Description</p>
        <textarea value={description} onChange={(e) => setDescription(e.target.value)} />
      </div>

      <div>
        <p>Device Location (Building & Floor)</p>

      </div>

      <div>
        <button>Create</button>
        <button onClick={() => setActiveMenu(MenuOptions.MainMenu)}>Cancel</button>
      </div>
    </div>
  );
};

export default Registration;
