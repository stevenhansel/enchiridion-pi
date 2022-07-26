import { useCallback, useEffect, useState } from 'react'

import { useCarousel } from './hooks';
import { invoke } from '@tauri-apps/api';

enum ApplicationErrorCode {
  InitializationError = 'INITIALIZATION_ERROR',
}

type ApplicationError = {
  code: ApplicationErrorCode,
  message: string,
}

const CAROUSEL_INTERVAL = 1000;

function App() {
  const [images, setImages] = useState<string[]>([]);
  const [error, setError] = useState<ApplicationError | null>(null);

  const { index, startCarousel, stopCarousel } = useCarousel(CAROUSEL_INTERVAL, images.length - 1);

  const initialize = useCallback(async () => {
    try {
      const images: string[] = await invoke('get_images');
      setImages(images);
    } catch (e) {
      setError({ 
        code: ApplicationErrorCode.InitializationError,
        message: 'Something went wrong when initializing the application',
      });
    }
  }, []);

  useEffect(() => {
    initialize().then(() => {
      startCarousel();
    });

    return () => {
      stopCarousel();
    }
  }, []);

  return (
    <div>
      <div className="container">
        {images.length > 0 ? (
          <img className="image" src={images[index]} />
        ) : null}
      </div>
      {error !== null ? (
        <div>
          <p>Error Code: {error.code}</p>
          <p>{error.message}</p>
        </div>
      ) : null}
    </div>
  )
}

export default App
