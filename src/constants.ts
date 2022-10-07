export enum ApplicationErrorCode {
  InitializationError = 'INITIALIZATION_ERROR',
  AuthenticationFailed = 'AUTHENTICATION_FAILED',
  DeviceNotFound = "DEVICE_NOT_FOUND",
  DeviceLinkingError = "DEVICE_LINKING_ERROR",
  ApplicationError = "APPLICATION_ERROR",
}

export type ApplicationError = {
  code: ApplicationErrorCode,
  message: string,
}

export const CAROUSEL_INTERVAL = 1000;
export const NETWORK_CONNECTION_CHECK_MS = 3000;
