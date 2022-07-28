import React, { useContext, useEffect, useState } from 'react';
import { Building, Floor, tauri } from '../tauri';
import { MenuContext, MenuOptions } from './constants';

const Registration = () => {
  const { setActiveMenu } = useContext(MenuContext);

  const [isBuildingLoading, setIsBuildingLoading] = useState(true);
  const [_isFloorLoading, setIsFloorLoading] = useState(false);

  const [_setIsFloorLoading, setError] = useState('');

  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [selectedBuildingId, setSelectedBuildingId] = useState<number|null>(null)
  const [selectedFloorId, setSelectedFloorId] = useState<number | null>(null);

  const [buildings, setBuildings] = useState<Building[]>([]);
  const [floors, setFloors] = useState<Floor[]>([]);

  const validateForm = (): boolean => {
    if (name === '') {
      return false;      
    }
    if (description === '') {
      return false;
    }
    if (selectedFloorId === null) {
      return false;
    } 

    return true;
  };

  const handleSubmitForm = async () => {
    if (!validateForm()) return;

    try {
      await tauri.createDevice({
        name,
        description,
        floorId: selectedFloorId as number,
        isLinked: true,
      });
      setActiveMenu(MenuOptions.MainMenu);
    } catch (e) {
      setError('Something went wrong when creating the device');
    }

  };

  const fetchBuildings = async (): Promise<void> => {
    try {
      const buildings = await tauri.getBuildings();
      if (buildings.length > 0) {
        setSelectedBuildingId(buildings[0].id);
      }

      setBuildings(buildings);
    } catch (e) {
      setError('Something went wrong when getting the buildings data');
    }

      setIsBuildingLoading(false);
  };

  const fetchFloors = async (buildingId: number): Promise<void> => {
    try {
      setIsFloorLoading(true);

      const floors = await tauri.getFloors(buildingId);
      if (floors.length > 0) {
        setSelectedFloorId(floors[0].id);
      }

      setFloors(floors);
      setIsFloorLoading(false);
    } catch (e) {
      setError('Something went wrong when getting the floors data');
    }
  };

  useEffect(() => {
    fetchBuildings();
  }, []);

  useEffect(() => {
    if (selectedBuildingId !== null) {
      fetchFloors(selectedBuildingId);
    }
  }, [selectedBuildingId])

  return (
    <div>
      {!isBuildingLoading ? (
        <>
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
            {selectedBuildingId !== null ? (
              <div>
                <span>Building:</span>
                <select value={selectedBuildingId} onChange={(e) => {
                  setSelectedBuildingId(parseInt(e.target.value))
                }}>
                    {buildings.map((building) => (
                      <option key={building.id} value={building.id}>{building.name}</option>
                    ))}
                </select>
              </div>
            ) : null}

            {selectedFloorId !== null ? (
              <div>
                <span>Floor:</span>
                <select value={selectedFloorId} onChange={(e) => setSelectedFloorId(parseInt(e.target.value))}>
                  {floors.map((floor) => (
                    <option key={floor.id} value={floor.id}>{floor.name}</option>
                  ))}
                </select>
              </div>
            ) : null}

          </div>

          <div>
            <button onClick={() => handleSubmitForm()}>Create</button>
            <button onClick={() => setActiveMenu(MenuOptions.MainMenu)}>Cancel</button>
          </div>
        </>
      ) : <p>Loading...</p>}
    </div>
  );
};

export default Registration;
