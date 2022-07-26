import { useEffect, useState } from 'react'

import { useCarousel } from './hooks';

const CAROUSEL_INTERVAL = 1000;
function App() {
  const [images, setImages] = useState<string[]>(['https://bm5cdn.azureedge.net/banner/20220722105843BOS00000029.jpg', 'https://bm5cdn.azureedge.net/banner/20220712175741BN123816140.jpg', 'https://bm5cdn.azureedge.net/banner/20220627154212BOS00000029.jpg']);
  const { index, startCarousel, stopCarousel } = useCarousel(CAROUSEL_INTERVAL, images.length - 1);

  useEffect(() => {
    startCarousel();

    return () => {
      stopCarousel();
    }
  }, []);

  return (
    <div>
      <div className="container">
        <img className="image" src={images[index]} />
      </div>
    </div>
  )
}

export default App
