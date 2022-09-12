import { createContext } from "react";
import { DeviceInformation } from "./tauri";

type SetState<T> = React.Dispatch<React.SetStateAction<T>>;

export type ApplicationContextType = {
  device: DeviceInformation | null;
  setDevice: SetState<DeviceInformation | null>;

  loading: boolean;
  setLoading: SetState<boolean>;

  error: ApplicationError | null;
  setError: SetState<ApplicationError | null>;
};

export const ApplicationContext = createContext<ApplicationContextType>({
  device: null,
  setDevice: () => {},

  loading: false,
  setLoading: () => {},

  error: null,
  setError: () => {},
});

