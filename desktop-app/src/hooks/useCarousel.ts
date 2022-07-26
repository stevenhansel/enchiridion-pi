import { useEffect, useRef, useState } from "react";

const useCarousel = (interval: number, max: number) => {
  const [index, setIndex] = useState(0);
  const [isStart, setIsStart] = useState(false);

  const isFirstStart = useRef(true)
  const tick = useRef(0);

  const startCarousel = () => setIsStart(true);

  const stopCarousel = () => setIsStart(false);

  useEffect(() => {
    if (isFirstStart.current === true) {
      isFirstStart.current = false;
      return;
    }

    if (isStart === true) {
      tick.current = setInterval(() => {
        setIndex((previousIndex) => previousIndex === max ? 0 : previousIndex + 1);
      }, interval);
    } else {
      clearInterval(tick.current);
    }

    return () => clearInterval(tick.current);
  }, [isStart]);

  return { index, startCarousel, stopCarousel };
};

export default useCarousel;
