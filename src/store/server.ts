import { writable, type Subscriber, type Writable, get } from 'svelte/store';
import { URL } from 'url';
import { invoke } from '@tauri-apps/api';
import { error } from 'console';

export enum Protocol {
    http = "http://",
    https = "https://"
}

export interface ServerParams {
    protocol: Protocol,
    address: string,
    port: number,
    token: string
}

interface ServerConfig {
    url: string,
    token: string
}

interface AppConfig {
    default: boolean
}

interface ServerInstance {
    display_name: string,
    server: ServerConfig,
    app: AppConfig | null,
}

type Config = Map<string, string>

class InstanceList {
    _list: Writable<Config>

    constructor() {
        this._list = writable<Config>(new Map)
    }

    get list() {
        return (async () => {
            await this.updateList();
            return this._list;
        })();
    }

    async add(server: ServerParams, app?: AppConfig | null) {
        try {
            const entry = await invoke("add_instance", { server: { ...server }, app: null, })
            await this.updateList()
        } catch (err) {
            alert(err)
        }
    }

    async remove(id: string) {
        try {
            await invoke("remove_instance", { id: id })
            await this.updateList()
        } catch (err) {
            alert(err)
        }
    }

    async updateList() {
        try {
            const list = await invoke("get_list")
            const map = new Map(Object.entries(list))
            this._list.set(map);
        } catch (err) {
            alert(err)
        }
    }

    subscribe(run: Subscriber<Config>) {
        return this._list.subscribe(run)
    }

}

export const list = new InstanceList();