import { useCallback, useEffect, useState } from 'react'

import { useCarousel } from './hooks';
import { invoke } from '@tauri-apps/api';
import { Menu } from './components';

enum ApplicationErrorCode {
  InitializationError = 'INITIALIZATION_ERROR',
}

enum ApplicationCommands {
  CreateDevice = "create",
}

type ApplicationError = {
  code: ApplicationErrorCode,
  message: string,
}

const CAROUSEL_INTERVAL = 1000;

function App() {
  const [images, setImages] = useState<string[]>([]);
  const [error, setError] = useState<ApplicationError | null>(null);

  const [isMenuOpen, setIsMenuOpen] = useState(false);

  const { index, startCarousel, stopCarousel } = useCarousel(CAROUSEL_INTERVAL, images.length - 1);

  const handleCommandKeydownListener = useCallback((e: KeyboardEvent) => {
    if (e.key === 'm') {
      setIsMenuOpen(true);
    } else if (e.key === 'Escape') {
      e.preventDefault();
      setIsMenuOpen(false);
    }
  }, []);

  const getAnnouncementMedias = useCallback(async () => {
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
    getAnnouncementMedias().then(() => {
      startCarousel();
    });

    return () => {
      stopCarousel();
    }
  }, []);

  useEffect(() => {
    window.addEventListener('keypress', handleCommandKeydownListener);

    return () => {
      window.removeEventListener('keypress', handleCommandKeydownListener);
    }
  }, [])

  return (
    <div>
      <div className="container">
        <img className="image" src={images.length > 0 ? images[index] : "/binus.jpeg"} />
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
