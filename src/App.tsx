import { CircularProgress, Snackbar } from "@mui/material";
import { useEffect, useState } from "react";
import { ApplicationError } from "./constants";
import { ApplicationContext } from "./context";

import { Authentication, Display } from "./pages";
import { DeviceInformation, getDeviceInformation } from "./tauri";

const App = () => {
  const [device, setDevice] = useState<DeviceInformation | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<ApplicationError | null>(null);

  useEffect(() => {
    setLoading(true);

    getDeviceInformation()
      .then((device) => {
        setLoading(false);
        setDevice(device);
      })
      .catch(() => setLoading(false));
  }, []);

  return (
    <ApplicationContext.Provider
      value={{ device, setDevice, loading, setLoading, error, setError }}
    >
      <div className="application-container">
        {loading ? (
          <CircularProgress />
        ) : (
          <>
            {device === null ? <Authentication /> : <Display />}

            <Snackbar
              anchorOrigin={{ vertical: "bottom", horizontal: "center" }}
              open={!!error}
              onClose={() => setError(null)}
              message={error?.message}
              key="application-error"
            />
          </>
        )}
      </div>
    </ApplicationContext.Provider>
  );
};

export default App;
