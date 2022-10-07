import { createContext } from "react";
import { ApplicationError } from "./constants";
import { CarouselState, defaultCarouselState } from "./hooks";
import { DeviceInformation } from "./tauri";

type SetState<T> = React.Dispatch<React.SetStateAction<T>>;

export type ApplicationContextType = {
  device: DeviceInformation | null;
  setDevice: SetState<DeviceInformation | null>;

  loading: boolean;
  setLoading: SetState<boolean>;

  error: ApplicationError | null;
  setError: SetState<ApplicationError | null>;

  isNetworkConnected: boolean;
  setIsNetworkConnected: SetState<boolean>;

  carousel: CarouselState;
};

export const ApplicationContext = createContext<ApplicationContextType>({
  device: null,
  setDevice: () => {},

  loading: false,
  setLoading: () => {},

  error: null,
  setError: () => {},

  isNetworkConnected: true,
  setIsNetworkConnected: () => {},

  carousel: defaultCarouselState(),
});
