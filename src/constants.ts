enum ApplicationErrorCode {
  InitializationError = 'INITIALIZATION_ERROR',
}

type ApplicationError = {
  code: ApplicationErrorCode,
  message: string,
}

const CAROUSEL_INTERVAL = 3000;

