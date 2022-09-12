import { useEffect, useRef, useState } from "react";

const useCarousel = (interval: number) => {
  const [index, setIndex] = useState(0);
  const [max, setMax] = useState<number|null>(null);
  const [isStart, setIsStart] = useState(false);

  const isFirstStart = useRef(true)
  const tick = useRef(0);

  const startCarousel = (newMax: number) => {
    setMax(newMax);
    setIsStart(true);
  };

  const stopCarousel = () => setIsStart(false);

  useEffect(() => {
    if (isFirstStart.current === true) {
      isFirstStart.current = false;
      return;
    }

    if (isStart === true) {
      tick.current = setInterval(() => {
        setIndex((previousIndex) => previousIndex + 1 === max ? 0 : previousIndex + 1);
      }, interval);
    } else {
      clearInterval(tick.current);
    }

    return () => clearInterval(tick.current);
  }, [max, isStart]);

  return { index, startCarousel, stopCarousel };
};

export default useCarousel;
