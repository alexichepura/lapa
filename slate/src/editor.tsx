import React, {
    CSSProperties,
  FC,
  HTMLAttributes,
  MouseEvent,
  PropsWithChildren,
  Ref,
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import {
  createEditor,
  Descendant,
  Editor,
  Transforms,
  Element as SlateElement,
  BaseEditor,
  Range,
  Location,
  NodeEntry,
  Node,
  Text,
  BaseRange,
  Span,
} from "slate";
import {
  Editable,
  ReactEditor,
  Slate,
  useFocused,
  useSelected,
  useSlate,
  withReact,
} from "slate-react";
import { debounce, DebounceCancelSignal } from "./debounce";
import { HistoryEditor, withHistory } from "slate-history";
import isHotkey from "is-hotkey";
import ReactDOM from "react-dom";
import isUrl from "is-url";
import { jsx } from "slate-hyperscript";

// https://www.slatejs.org/examples/richtext
// https://github.com/ianstormtaylor/slate/blob/main/site/examples/ts/richtext.tsx
// https://www.slatejs.org/examples/hovering-toolbar
// https://github.com/ianstormtaylor/slate/blob/main/site/examples/ts/hovering-toolbar.tsx
// https://www.slatejs.org/examples/paste-html
// https://github.com/ianstormtaylor/slate/blob/main/site/examples/ts/paste-html.tsx

export type CustomEditor = BaseEditor & ReactEditor & HistoryEditor;
const P = "p";
const H1 = "h1";
const H2 = "h2";
const H3 = "h3";
const H4 = "h4";
const BLOCKQUOTE = "blockquote";
const UL = "ul";
const OL = "ol";
const LI = "li";
const A = "a";
const IMG = "img";
const LIST_TYPES = [OL, UL];
// const TEXT_ALIGN_TYPES = ["left", "center", "right", "justify"];
const HOTKEYS = {
  "mod+b": "strong",
  "mod+i": "em",
  "mod+u": "u",
  "mod+`": "code",
};

export type PElement = {
  type: typeof P;
  children: CustomText[];
};
export type H1Element = {
  type: typeof H1;
  children: CustomText[];
};
export type H2Element = {
  type: typeof H2;
  children: CustomText[];
};
export type H3Element = {
  type: typeof H3;
  children: CustomText[];
};
export type H4Element = {
  type: typeof H4;
  children: CustomText[];
};
export type BlockquoteElement = {
  type: typeof BLOCKQUOTE;
  children: CustomText[];
};
export type UlElement = {
  type: typeof UL;
  children: CustomText[];
};
export type OlElement = {
  type: typeof OL;
  children: CustomText[];
};
export type LiElement = {
  type: typeof LI;
  children: CustomText[];
};
export type AElement = {
  type: typeof A;
  attributes: {
    href: string;
  };
  children: CustomText[];
};
export type ImgElement = {
  type: typeof IMG;
  id: string;
  alt: string;
  caption: string;
};
export type CustomElement =
  | PElement
  | H1Element
  | H2Element
  | H3Element
  | H4Element
  | BlockquoteElement
  | UlElement
  | OlElement
  | LiElement
  | AElement
  | ImgElement;
export type FormattedText = {
  text: string;
  strong?: true;
  mark?: true;
  code?: true;
  em?: true;
  u?: true;
};
export type CustomText = FormattedText;

declare module "slate" {
  interface CustomTypes {
    Editor: CustomEditor;
    Element: CustomElement;
    Text: CustomText;
  }
}

export type TOnChange = (model: Descendant[]) => void;
export type TEditLink = (link_edit_data: LinkEditData, on_save: (link_edit_data: LinkEditData) => void) => void;
export type TEditImage = (image_edit_data: ImageEditData, on_save: (image_edit_data: ImageEditData) => void) => void;
type TRichTextFieldProps = {
  initialValue: Descendant[];
  callback: (model_json: string) => void;
  edit_link: TEditLink
  edit_image: TEditImage
  set_callbacks: (callbacks: TCallbacks) => void; 
};
export type TCallbacks = {
  link_del: () => void;
};

export const SlateEditor: FC<TRichTextFieldProps> = ({
  initialValue,
  callback,
  edit_link,
  edit_image,
  set_callbacks
}) => {
  const [editor] = useState(() =>
    withHtml(withReact(withHistory(createEditor()))),
  );
  const refV = useRef<Descendant[]>([]);
  const d = useMemo(() => {
    const df = () => {
      callback(JSON.stringify(refV.current));
    };
    return debounce(df, 300);
  }, []);
  useEffect(() => () => d(new DebounceCancelSignal()), []);

  const callbacks = useMemo(() => {
    const link_del = () => {
        Transforms.unwrapNodes(editor, {
          match: (e) => "type" in e && e.type === A,
        })
    };
    return { link_del }
  }, []);
  useEffect(() => {
    set_callbacks(callbacks);
  }, []);

  const renderLeaf = useCallback((props) => <Leaf {...props} />, []);
  const renderElement = useCallback((props) => <Element {...props} />, []);
  return (
    <Slate
      editor={editor}
      onChange={(v) => {
        refV.current = v;
        d(v);
      }}
      initialValue={initialValue}
    >
      <HoveringToolbar edit_link={edit_link} edit_image={edit_image}/>
      <Toolbar edit_link={edit_link} edit_image={edit_image}/>
      <Editable
        renderElement={renderElement}
        renderLeaf={renderLeaf}
        placeholder="Placeholder..."
        spellCheck
        onKeyDown={(event) => {
          for (const hotkey in HOTKEYS) {
            if (isHotkey(hotkey, event as any)) {
              event.preventDefault();
              const mark = HOTKEYS[hotkey];
              toggleMark(editor, mark);
            }
          }
        }}
      />
    </Slate>
  );
};

const Element: FC<
  PropsWithChildren & {
    attributes: HTMLAttributes<HTMLElement>;
    element: CustomElement;
  }
> = ({ attributes, children, element }) => {
  switch (element.type) {
    case P:
      return <p {...attributes}>{children}</p>;
    case H1:
      return <h1 {...attributes}>{children}</h1>;
    case H2:
      return <h2 {...attributes}>{children}</h2>;
    case H3:
      return <h3 {...attributes}>{children}</h3>;
    case H4:
      return <h4 {...attributes}>{children}</h4>;
    case BLOCKQUOTE:
      return <blockquote {...attributes}>{children}</blockquote>;
    case OL:
      return <ol {...attributes}>{children}</ol>;
    case UL:
      return <ul {...attributes}>{children}</ul>;
    case LI:
      return <li {...attributes}>{children}</li>;
    case A:
      return (
        <a {...attributes} href={element.attributes.href}>
          {children}
        </a>
      );
    case IMG:
      const selected = useSelected()
      const focused = useFocused()
      const style: CSSProperties = {
        display: "inline-block",
        fontSize: 0,
        boxShadow: `${focused && selected ? "0 0 0 3px #B4D5FF" : "none"}`,
        position: "relative",
      }
      const src = `/cdn/` + element.id;
      return (
        <div {...attributes}>
          <div contentEditable={false} style={style}>
            <img src={src} alt={element.alt} />
            <span>{element.caption}</span>
          </div>
          {children}
        </div>
      );
  }
};

const Leaf: FC<
  PropsWithChildren & {
    attributes: any;
    leaf: FormattedText;
  }
> = ({ attributes, children, leaf }) => {
  if (leaf.mark) {
    children = <mark>{children}</mark>;
  }
  if (leaf.strong) {
    children = <strong>{children}</strong>;
  }
  if (leaf.code) {
    children = <code>{children}</code>;
  }
  if (leaf.em) {
    children = <em>{children}</em>;
  }
  if (leaf.u) {
    children = <u>{children}</u>;
  }
  return <span {...attributes}>{children}</span>;
};

const toggleMark = (editor, format) => {
  const isActive = isMarkActive(editor, format);

  if (isActive) {
    Editor.removeMark(editor, format);
  } else {
    Editor.addMark(editor, format, true);
  }
};
const toggleBlock = (editor, format) => {
  const isActive = isBlockActive(
    editor,
    format,
    "type",
    // TEXT_ALIGN_TYPES.includes(format) ? "align" : "type",
  );
  const isList = LIST_TYPES.includes(format);

  Transforms.unwrapNodes(editor, {
    match: (n) =>
      !Editor.isEditor(n) &&
      SlateElement.isElement(n) &&
      LIST_TYPES.includes(n.type), // &&
    // !TEXT_ALIGN_TYPES.includes(format),
    split: true,
  });
  let newProperties: Partial<SlateElement>;
  // if (TEXT_ALIGN_TYPES.includes(format)) {
  //   newProperties = {
  //     align: isActive ? undefined : format,
  //   };
  // } else {
  newProperties = {
    type: isActive ? P : isList ? LI : format,
  };
  // }
  Transforms.setNodes<SlateElement>(editor, newProperties);

  if (!isActive && isList) {
    const block = { type: format, children: [] };
    Transforms.wrapNodes(editor, block);
  }
};

const isMarkActive = (editor, format) => {
  const marks = Editor.marks(editor);
  return marks ? marks[format] === true : false;
};
const isBlockActive = (editor, format, blockType = "type") => {
  const { selection } = editor;
  if (!selection) return false;
  const [match] = Array.from(
    Editor.nodes(editor, {
      at: Editor.unhangRange(editor, selection),
      match: (n) =>
        !Editor.isEditor(n) &&
        SlateElement.isElement(n) &&
        n[blockType] === format,
    }),
  );
  return !!match;
};
const isLinkActive = (editor: CustomEditor) => {
  const [link] = Editor.nodes(editor, {
    match: (n) =>
      !Editor.isEditor(n) && SlateElement.isElement(n) && n.type === A,
  });
  return !!link;
};

const BlockButton: FC<{ icon: string; format: string }> = ({
  format,
  icon,
}) => {
  const editor = useSlate();
  return (
    <Button
      active={isBlockActive(
        editor,
        format,
        "type",
        // TEXT_ALIGN_TYPES.includes(format) ? "align" : "type",
      )}
      onMouseDown={(event) => {
        event.preventDefault();
        toggleBlock(editor, format);
      }}
    >
      {icon}
    </Button>
  );
};

const MarkButton: FC<{ icon: string; format: string }> = ({ format, icon }) => {
  const editor = useSlate();
  return (
    <Button
      active={isMarkActive(editor, format)}
      onMouseDown={(event) => {
        event.preventDefault();
        toggleMark(editor, format);
      }}
    >
      {icon}
    </Button>
  );
};

type TButtonProps = PropsWithChildren & {
  active: boolean;
  onMouseDown: (event: MouseEvent) => void;
};
export const Button = React.forwardRef<HTMLButtonElement, TButtonProps>(
  ({ active, ...props }, ref) => (
    <button
      {...props}
      ref={ref}
      // style={active ? { boxShadow: "var(--inner-shadow-4)" } : {}}
      style={active ? { backgroundColor: "var(--surface-4)" } : {}}
    />
  ),
);

type TToolBarProps = {
  edit_link: TEditLink  
  edit_image: TEditImage  
}
export const Toolbar: FC<TToolBarProps> = ({edit_link, edit_image}) => {
  return (
    <div>
      <MarkButton format="strong" icon="𝗕" />
      <MarkButton format="em" icon="𝑰" />
      <MarkButton format="u" icon="⎁" />
      <MarkButton format="code" icon="∁" />
      <AButton edit_link={edit_link}/>
      <ImageButton edit_image={edit_image}/>
      <BlockButton format={H1} icon="𝖧𝟣" />
      <BlockButton format={H2} icon="𝖧𝟤" />
      <BlockButton format={H3} icon="𝖧𝟥" />
      <BlockButton format={H4} icon="𝖧𝟦" />
      <BlockButton format="blockquote" icon="❞" />
      <BlockButton format={OL} icon="№" />
      <BlockButton format={UL} icon="⋮" />
      {/* <BlockButton format="left" icon="⇤" />
      <BlockButton format="center" icon="⤄" />
      <BlockButton format="right" icon="⇥" />
      <BlockButton format="justify" icon="⟺" /> */}
    </div>
  );
};

export const Portal: FC<PropsWithChildren> = ({ children }) => {
  return typeof document === "object"
    ? ReactDOM.createPortal(children, document.body)
    : null;
};
const HoveringToolbar: FC<TToolBarProps> = (props) => {
  // https://www.slatejs.org/examples/hovering-toolbar
  const ref = useRef<HTMLDivElement | null>(null);
  const editor = useSlate();
  const inFocus = useFocused();

  useEffect(() => {
    const el = ref.current;
    const { selection } = editor;

    if (!el) {
      return;
    }

    if (
      !selection ||
      !inFocus ||
      Range.isCollapsed(selection) ||
      Editor.string(editor, selection) === ""
    ) {
      el.style.top = "-10000px";
      el.style.left = "-10000px";
      el.style.opacity = "0";
      return;
    }

    const domSelection = window.getSelection()!;
    const domRange = domSelection.getRangeAt(0);
    const rect = domRange.getBoundingClientRect();
    el.style.opacity = "1";
    el.style.top = `${rect.top + window.scrollY - el.offsetHeight}px`;
    const left = Math.max(
      rect.left + window.scrollX - el.offsetWidth / 2 + rect.width / 2,
      0,
    );
    el.style.left = `${left}px`;
  });

  return (
    <Portal>
      <div
        ref={ref}
        style={{
          position: "absolute",
          zIndex: 1,
          top: "-10000px",
          left: "-10000px",
          opacity: "0",
          transition: "opacity 0.3s",
          marginTop: "-6px",
        }}
        // className={css`
        //   padding: 8px 7px 6px;
        //   background-color: #222;
        //   border-radius: 4px;
        // `}
        onMouseDown={(e) => {
          // prevent toolbar from taking focus away from editor
          e.preventDefault();
        }}
      >
        <Toolbar {...props}/>
      </div>
    </Portal>
  );
};
const withHtml = (editor: CustomEditor) => {
  const { insertData, insertText, isInline, isVoid, normalizeNode } = editor
  editor.isInline = (element) =>
    [A].includes(element.type) || isInline(element);
    editor.isVoid = element => {
      return element.type === 'img' ? true : isVoid(element)
    }
  editor.insertText = (text) => {
    if (text && isUrl(text)) {
      wrapLink(editor, text);
    } else {
      insertText(text);
    }
  };
  editor.insertData = (data) => {
    const text = data.getData("text/plain");
    if (text && isUrl(text)) {
      wrapLink(editor, text);
    } else {
      const html = data.getData('text/html')
      if (html) {
        const parsed = new DOMParser().parseFromString(html, 'text/html')
        const fragment = deserialize(parsed.body)
        Transforms.insertFragment(editor, fragment)
        return
      }
      insertData(data)
    }
  };
  // editor.normalizeNode = (entry) => {
  //     const [n, p] = entry
  //     // If the element is a block (except ul or ol), ensure it's children are valid.
  //     if (isNodeHtmlBlockEl(n) && !isNodeListEl(n)) {
  //       for (const [child, childPath] of Node.children(editor, p)) {
  //         // if (isNodeTable2(child)) {
  //         //   console.info("slate:normalise:liftNodes", n, child, childPath)
  //         //   Transforms.liftNodes(editor, { at: childPath })
  //         //   return
  //         // }
  //         if (Element.isElement(child) && !editor.isInline(child)) {
  //           console.info("slate:normalise:unwrapNodes", n, child, childPath)
  //           Transforms.unwrapNodes(editor, { at: childPath })
  //           return
  //         }
  //       }
  //     }

  //     // Fall back to the original `normalizeNode` to enforce other constraints.
  //     normalizeNode(entry)
  //   }

  return editor
}
const ELEMENT_TAGS: Record<string, (el: HTMLElement | ChildNode) => CustomElement> = {
  A: (el) => ({ type: A, attributes: {href: (el as HTMLAnchorElement).getAttribute('href') || ""}, children: [] }),
  BLOCKQUOTE: () => ({ type: BLOCKQUOTE, children: [] }),
  H1: () => ({ type: H1, children: [] }),
  H2: () => ({ type: H2, children: [] }),
  H3: () => ({ type: H3, children: []  }),
  H4: () => ({ type: H4, children: [] }),
  // H5: () => ({ type: 'heading-five' }),
  // H6: () => ({ type: 'heading-six' }),
  // IMG: el => ({ type: 'image', url: el.getAttribute('src') }),
  LI: () => ({ type: LI, children: [] }),
  OL: () => ({ type: OL, children: [] }),
  P: () => ({ type: P, children: [] }),
  // PRE: () => ({ type: CODE, children: [] }),
  UL: () => ({ type: OL, children: [] }),
}
// COMPAT: `B` is omitted here because Google Docs uses `<b>` in weird ways.
const TEXT_TAGS = {
  CODE: () => ({ code: true }),
  // DEL: () => ({ strikethrough: true }),
  EM: () => ({ em: true }),
  I: () => ({ em: true }),
  // S: () => ({ strikethrough: true }),
  STRONG: () => ({ strong: true }),
  U: () => ({ u: true }),
}
export const deserialize = (el: HTMLElement | ChildNode): any => {
  if (el.nodeType === 3) {
    return el.textContent
  } else if (el.nodeType !== 1) {
    return null
  } else if (el.nodeName === 'BR') {
    return '\n'
  }

  const fragmentEl = (el as HTMLElement).querySelector("[data-slate-fragment]")
  if (fragmentEl) {
    const fragmentBase64 = (fragmentEl as HTMLElement).dataset?.slateFragment
    if (fragmentBase64) {
      try {
        const model: Descendant[] = JSON.parse(
          decodeURIComponent(atob(fragmentBase64))
        )
        console.log("fragment model", model)
        return model
      } catch (e) {
        console.error("json parse error", e)
      }
    }
  }
  
  const { nodeName } = el
  let parent = el
  if (
    nodeName === 'PRE' &&
    el.childNodes[0] &&
    el.childNodes[0].nodeName === 'CODE'
  ) {
    parent = el.childNodes[0] as HTMLElement
  }
  let children = Array.from(parent.childNodes).map(deserialize).flat()
  if (children.length === 0) {
    children = [{ text: '' }]
  }

  if (el.nodeName === 'BODY') {
    return jsx('fragment', {}, children)
    // let html_el = el as HTMLElement;
    // const firstElementChild =
    //   html_el.children &&
    //   Array.from(html_el.children).filter((child) => child.nodeName !== "META")[0]
    // if (firstElementChild && firstElementChild.nodeName === "B") {
    //   // chrome adds <b> wrapper
    //   // also <br class="Apple-interchange-newline" /> is eliminated by this logic
    //   // return fromHtmlChildNodes(firstElementChild.childNodes)
    //   return jsx('fragment', {}, firstElementChild.childNodes)
    // }
    // // return fromHtmlChildNodes(html_el.children)
    // return jsx('fragment', {}, html_el.children)

  }
  if (el.nodeName === 'BR') {
    return { text: "\n" }
  }

  if (ELEMENT_TAGS[nodeName]) {
    const attrs = ELEMENT_TAGS[nodeName](el)
    return jsx('element', attrs, children)
    // const element = ELEMENT_TAGS[nodeName](el)
    // return {...element, children }
  }
  if (TEXT_TAGS[nodeName]) {
    const attrs = TEXT_TAGS[nodeName](el)
    return children.map(child => jsx('text', attrs, child))
    // const attrs = TEXT_TAGS[nodeName](el)
    // const marks = children.map((child) => {
    //   return { ...attrs, ...child }
    // })
    // return marks

    // return children.map(child => jsx('text', attrs, child))
  }
  console.log("missing", el)

  return children
}
// const fromHtmlChildNodes = (nodes: NodeListOf<ChildNode> | HTMLCollection) => {
//   const r = Array.from(nodes)
//     .map((el) => deserialize(el as HTMLElement))
//     .flat()
//   // console.log("fromHtmlChildNodes", r)
//   return r
// }

// Anchor (link)
type LinkEditData = {
    href: string,
    text: string,
}
type TAnchorProps = {
  edit_link: TEditLink  
}
const AButton: FC<TAnchorProps> = ({edit_link}) => {
  const editor = useSlate();
  return (
    <Button
      active={isLinkActive(editor)}
      onMouseDown={(event) => {
        event.preventDefault();
        let initial_data = getInitialLinkData(editor, A);
        const { at } = initial_data;
        let initial_link = initial_data.link;
        edit_link(initial_link, (link) => {
          console.log("on_saved", link)
          if (!at) throw new Error("Invalid range. Must be typeof Range.")
          setLink({ editor, at, link })
        });
      }}
    >
      🔗
    </Button>
  );
};
type TWrapLinkCommand = {
  link: LinkEditData
  at?: Location
  editor: CustomEditor
}
const setLink = ({ editor, link, at }: TWrapLinkCommand): void => {
  let isCollapsed: boolean = true
  if (Range.isRange(at)) {
    isCollapsed = Range.isCollapsed(at)
  }
  const text = link.text
  const _link: AElement = {
    type: "a",
    attributes: {
      href: link.href,
    },
    children: [{ text }],
  }

  const linkEntryAt = findLinkEntry(editor, A, at)
  if (linkEntryAt) {
    Transforms.setNodes(editor, _link, { at: linkEntryAt[1] })
    // setNodes doesn't change text or i don't know
    const [textEntryAt] = Editor.nodes<AElement>(editor, {
      match: (n) => Text.isText(n),
      at,
    })
    Transforms.delete(editor, { at: textEntryAt[1] })
    Transforms.insertText(editor, text, { at: textEntryAt[1] })
  } else {
    if (isCollapsed) {
      Transforms.insertNodes(editor, [link], { at })
    } else {
      Transforms.wrapNodes(editor, _link, { at, split: true })
    }
  }
}

const insertLink = (editor: CustomEditor, url: string) => {
  if (editor.selection) {
    wrapLink(editor, url);
  }
};
const unwrapLink = (editor: CustomEditor) => {
  Transforms.unwrapNodes(editor, {
    match: (n) =>
      !Editor.isEditor(n) && SlateElement.isElement(n) && n.type === "a",
  });
};
const wrapLink = (editor: CustomEditor, href: string) => {
  if (isLinkActive(editor)) {
    unwrapLink(editor);
  }

  const { selection } = editor;
  const isCollapsed = selection && Range.isCollapsed(selection);
  const link: AElement = {
    type: "a",
    attributes: {
      href: href,
    },
    children: isCollapsed ? [{ text: href }] : [],
  };

  if (isCollapsed) {
    Transforms.insertNodes(editor, link);
  } else {
    Transforms.wrapNodes(editor, link, { split: true });
    Transforms.collapse(editor, { edge: "end" });
  }
};
const findLinkEntry = (
  editor: Editor,
  t: typeof A,
  at?: Location
): NodeEntry<AElement> | null => {
  const [entry] = Editor.nodes<AElement>(editor, {
    match: (e) => "type" in e && e.type === t,
    at,
  })
  return entry
}
// const findLink = (editor: Editor, t: typeof A): AElement | null => {
//   const linkEntry = findLinkEntry(editor, t)
//   return linkEntry ? linkEntry[0] : null
// }

const getInitialLinkData = (
  ed: Editor,
  type: typeof A
): { at: BaseRange | null; isExpanded: boolean; link: LinkEditData } => {
  const linkEntry = findLinkEntry(ed, type)
  const link = linkEntry && linkEntry[0]
  const isExpanded = ed.selection ? Range.isExpanded(ed.selection) : false
  // const text =
  //   link && link.type === A
  //     ? link.txt
  //     : ed.selection && isExpanded
  //     ? Editor.string(ed, ed.selection)
  //     : (link && Node.string(link)) || ""
  const text =
      ed.selection && isExpanded
      ? Editor.string(ed, ed.selection)
      : (link && Node.string(link)) || ""

  return {
    at: ed.selection ? { ...ed.selection } : null,
    isExpanded,
    link: {
      text: text,
      href: link ? link.attributes.href : "",
    },
  }
}
// Image
type ImageEditData = {
    id: string,
    alt: string,
    caption: string,
}

type TImageProps = {
  edit_image: TEditImage 
}
const ImageButton: FC<TImageProps> = ({edit_image}) => {
  const editor = useSlate();
  return (
    <Button
      active={false}
      onMouseDown={(event) => {
        event.preventDefault();
        let at = editor.selection;
        let initial_image: ImageEditData = {id: "", alt: "", caption: ""};
        edit_image(initial_image, (image) => {
          console.log("image on_saved", image)
          let image_el: ImgElement = {
            type: IMG,
            id: image.id,
            alt: image.alt,
            caption: image.caption
          };
          insertBlock(editor, image_el, at as BaseRange)
        });
      }}
    >
      🏞️
    </Button>
  );
};
export const insertBlock = (
  editor: CustomEditor,
  slateElement: CustomElement,
  range: Range
) => {
  const [node] = Editor.node(editor, range)
  if (node && "text" in node && node.text === "") {
    const [parent] = Editor.parent(editor, range)
    Transforms.unsetNodes(editor, Object.keys(parent), { at: range })
    Transforms.setNodes<CustomElement>(editor, slateElement, { at: range })
  } else {
    Transforms.insertNodes(editor, slateElement, { at: range })
  }
}
