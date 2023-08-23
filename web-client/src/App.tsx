import Editor from "@monaco-editor/react";
import Parser, { Tree } from "web-tree-sitter";
import "./App.css";
import "./tailwind.css";
import { useState } from "react";

function App() {
  let ts: Parser;

  (async () => {
    await Parser.init();
    ts = new Parser();
    const Lang = await Parser.Language.load("tree-sitter-javascript.wasm");
    ts.setLanguage(Lang);
  })();

  const [request, setRequest] = useState<Tree | undefined>();

  function handleChange(value: string | undefined): void {
    if (value) {
      setRequest((r) => {
        try {
          return ts.parse(value);
        } catch (e) {
          // NOTE: we will sometimes quietly fail parsing. This should be
          // okay because we will always parse before executing and throw for real.
          return r;
        }
      });
    }
  }
  return (
    <>
      <h1>Playground</h1>
      <div className="grid grid-cols-2 gap-4 w-screen">
        <Editor
          height="90vh"
          theme="vs-dark"
          language="javascript"
          onChange={handleChange}
          options={{}}
        />
        <Editor
          height="90vh"
          theme="vs-dark"
          value={request?.rootNode.toString()}
          options={{ readOnly: true, dropIntoEditor: { enabled: false } }}
        />
      </div>
    </>
  );
}

export default App;
