import { useEffect, useRef, useState } from 'react';
import { MemoryRouter as Router, Routes, Route } from 'react-router-dom';
import './App.css';

const delay = 3 * 1000;

const Hello = () => {
  const [activeIndex, setActiveIndex] = useState<number>(0);
  const [images, setImages] = useState<string[]>();

  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  const resetTimeout = () => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }
  };

  useEffect(() => {
    loadImages();
  }, []);

  useEffect(() => {
    resetTimeout();

    timeoutRef.current = setTimeout(() => {
      setActiveIndex((previousIndex) => {
        if (!images) {
          return 0;
        }

        if (previousIndex === images.length - 1) {
          return 0;
        }

        return previousIndex + 1;
      });
    }, delay);

    return () => {
      resetTimeout();
    };
  }, [activeIndex, images]);

  const loadImages = async () => {
    const response = (await window.electron.ipcRenderer.invoke(
      'get-images'
    )) as any;

    setImages(response);
  };

  return (
    <div className="slideshow">
      <div
        className="slideshow-slider"
        style={{ transform: `translate3d(${-activeIndex * 100}%, 0, 0)` }}
      >
        {images &&
          images.map((image, i) => (
            <img className="slide" key={i} src={`enchridion://${image}`} />
          ))}
      </div>
    </div>
  );
};

export default function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Hello />} />
      </Routes>
    </Router>
  );
}
