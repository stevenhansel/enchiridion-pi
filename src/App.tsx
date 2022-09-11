import { createContext, useEffect, useState } from "react";
import { SyncLoader } from "react-spinners";
import { Authentication, Display } from "./pages";
import { DeviceInformation, getDeviceInformation } from "./tauri";

type SetState<T> = React.SetStateAction<React.Dispatch<T>>;

type ApplicationContextType = {
  device: DeviceInformation | null;
  setDevice: SetState<DeviceInformation | null>;

  loading: boolean;
  setLoading: SetState<boolean>;

  error: ApplicationError | null;
  setError: SetState<ApplicationError | null>;
};

const ApplicationContext = createContext<ApplicationContextType>({
  device: null,
  setDevice: () => {},

  loading: false,
  setLoading: () => {},

  error: null,
  setError: () => {},
});

const App = () => {
  const [device, setDevice] = useState<DeviceInformation | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<ApplicationError | null>(null);

  useEffect(() => {
    getDeviceInformation().then((device) => setDevice(device));
  }, []);

  return (
    <ApplicationContext.Provider
      value={{ device, setDevice, loading, setLoading, error, setError }}
    >
      <div className="application-container">
        {loading ? (
          <SyncLoader />
        ) : (
          <>
            {device === null ? <Authentication /> : <Display />}

            {error !== null ? (
              <div>
                <p>Error Code: {error.code}</p>
                <p>{error.message}</p>
              </div>
            ) : null}
          </>
        )}
      </div>
    </ApplicationContext.Provider>
  );
};

export default App;
