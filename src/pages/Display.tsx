import { useCallback, useEffect, useState } from 'react'

import { useCarousel } from '../hooks';
import { subscribeToAnnouncementUpdates, tauri } from '../tauri';

const Display = () => {
  const [images, setImages] = useState<string[]>([]);
  const [error, setError] = useState<ApplicationError | null>(null);

  const { index, startCarousel, stopCarousel } = useCarousel(CAROUSEL_INTERVAL);

  const getAnnouncementMedias = useCallback(async () => {
    try {
      stopCarousel();

      const images = await tauri.getImages();

      setImages(images);
      startCarousel(images.length);
    } catch (e) {
      setError({
        code: ApplicationErrorCode.InitializationError,
        message: "Something went wrong when initializing the application",
      });
    }
  }, []);

  const initialize = () => {
    const unlistener = getAnnouncementMedias()
      .then(() => {
        return subscribeToAnnouncementUpdates(() => getAnnouncementMedias());
      })
      .then((unlistener) => {
        return unlistener;
      });

    return unlistener;
  };

  useEffect(() => {
    initialize();
  }, []);

  return (
    <div>
      <div>
        <img
          className="image"
          src={images.length > 0 ? images[index] : "/binus.jpeg"}
        />
      </div>

    </div>
  );
};

export default Display;
