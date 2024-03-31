import { writable, type Subscriber, type Writable, get } from 'svelte/store';
import { URL } from 'url';
import { invoke } from '@tauri-apps/api';
import { error } from 'console';

interface HassEntity {
    entity_id: string,
    last_changed: string,
    state: string,
    attributes: any,
    last_updated: string,
    context: Context | undefined,
}

interface Context {
    id: string,
    parent_id: string | undefined,
    user_id: string | undefined,
}

type Entities = HassEntity[]

class InstanceList {
    _entities: Writable<Entities>

    constructor() {
        this._entities = writable<Entities>([])
    }

    get value() {
        return get(this._entities)
    }

    async update() {
        try {
            const entities = await invoke<Entities>("hass_states")
            this._entities.set(entities);

        } catch (err) {
            alert(err)
        }
    }

    subscribe(run: Subscriber<Entities>) {
        return this._entities.subscribe(run)
    }

}

export const entities = new InstanceList();