export class DebounceCancelSignal {}

export const debounce = (f: (...args: any) => void, ms: number) => {
  let timer: any = null;

  return function (this: any, ...args: any) {
    if (args[0] && args[0] instanceof DebounceCancelSignal) {
      console.log("debounce: received DebounceCancelSignal");
      if (timer !== null) clearTimeout(timer);
    }
    const onComplete = () => {
      f.apply(this, args);
      timer = null;
    };

    if (timer) {
      // console.log("debounce: reset");
      clearTimeout(timer);
    }
    // console.log("debounce: new timeout");
    timer = window.setTimeout(onComplete, ms);
  };
};
