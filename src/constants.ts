export enum ApplicationErrorCode {
  InitializationError = 'INITIALIZATION_ERROR',
  AuthenticationFailed = 'AUTHENTICATION_FAILED',
}

export type ApplicationError = {
  code: ApplicationErrorCode,
  message: string,
}

export const CAROUSEL_INTERVAL = 3000;

