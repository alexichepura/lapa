import { createRoot } from "react-dom/client";
import { CustomElement, SlateEditor, TCallbacks, TEditImage, TEditLink } from "./editor";
import { Descendant } from "slate";
import React from "react";

export const defaultSlateValue: CustomElement[] = [
  // { type: EHtmlBlock.p, children: [{ text: "" }] },
  { type: "p", children: [{ text: "" }] },
];

export const start = (
  model_json: string,
  callback: (model_json: string) => void,
  edit_link: TEditLink,
  edit_image: TEditImage,
  set_callbacks: (callbacks: TCallbacks) => void
) => {
  let initialValue: Descendant[];
  try {
    initialValue = JSON.parse(model_json);
  } catch (e) {
    console.error(e);
    initialValue = defaultSlateValue;
  }
  const root = createRoot(document.getElementById("app")!);

  root.render(<SlateEditor
    initialValue={initialValue}
    callback={callback}
    edit_link={edit_link}
    edit_image={edit_image}
    set_callbacks={set_callbacks}
  />);
};
export default {
  start,
};
// setTimeout(() => {
//   let content: string = (window as any).CONTENT;
//   // console.log("CONTENT=" + content);
//   let initialValue: Descendant[];
//   try {
//     initialValue = JSON.parse(content);
//   } catch (e) {
//     console.log(e);
//     initialValue = [];
//   }

//   document.body.innerHTML = '<div id="app"></div>';
//   const root = createRoot(document.getElementById("app")!);
//   root.render(<SlateEditor initialValue={initialValue} />);
// }, 10);
