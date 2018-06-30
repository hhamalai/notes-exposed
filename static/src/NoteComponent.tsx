import * as React from "react";
import {NoteInterface, Note} from './NoteInterface';
import './NoteStyles.css';

interface NotesState {
    notes: Note[]
}

export default class NoteComponent extends React.Component<NoteInterface, NotesState> {
    constructor (props: NoteInterface){
        super(props);
        this.state = {notes: []};
    }

    componentDidMount() {
        fetch('/api/notes')
            .then(results => {
                return results.json();
            })
            .then(data => {
                this.setState({notes: data});
            })
    }

    render() {
        return (
            <ul>
                {this.state.notes.map(function(note, i) {
                    if (note.url)
                        return <li key={ i }>{note.date}: <a href={note.url}>{note.url}</a></li>;
                    else
                        return <li key={ i }>{note.date}: {note.comment}</li>;
                })}
            </ul>
        );
    }
}