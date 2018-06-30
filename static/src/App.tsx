import * as React from "react";
import * as ReactDOM from "react-dom";
import NoteComponent from './NoteComponent';
import './index.css';
ReactDOM.render(
    <div>
        <h1>Notes</h1>
        <NoteComponent />
    </div>,
    document.getElementById("root")
);