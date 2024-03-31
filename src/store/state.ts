import { writable, type Subscriber, type Writable, get } from 'svelte/store';
import { invoke } from '@tauri-apps/api';

interface Connected {
    Connected: string
}

type State = "Disconnected" | Connected

class StateStore {
    _state: Writable<State>

    constructor() {
        this._state = writable<State>()
    }

    get value() {
        return get(this._state)
    }

    async update() {
        try {
            const newState = await invoke<State>("get_status");
            this._state.set(newState);
        } catch (err) {
            alert(err)
        }
    }

    subscribe(run: Subscriber<State>) {
        return this._state.subscribe(run)
    }

}

export const state = new StateStore();