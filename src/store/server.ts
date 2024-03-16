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

// class Server {
//     protocol: Protocol
//     port: number
//     address: string
//     token: string
//     // name: string
//     // url: URL
    
//     constructor(login: ServerParams) {
//         this.protocol = login.protocol
//         this.address = login.address
//         this.port = login.port
//         this.token = login.token
//         // this.name = `${this.address}:${this.port}`
//         // this.url = new URL(`${this.protocol}${this.address}:${this.port}`)
//     }

//     get name() {
//         return `${this.address}:${this.port}`
//     }

//     get url() {
//         return new URL(`${this.protocol}${this.address}:${this.port}`)
//     }
// }

type Config = Map<string, string>

class InstanceList {
    _list: Writable<Config>

    constructor() {
        this._list = writable<Config>(new Map)
    }

    get list () {
        return (async () => {
            await this.updateList();
            return this._list;
        })();
    }

    async add(server: ServerParams, app?: AppConfig | null) {
        try {
            const entry = await invoke("add_instance", {server: {...server}, app: null, })
            await this.updateList()
        } catch(err) {
            alert(err)
        }
    }

    async remove(id: string) {
        await invoke("remove_instance", {id: id})
        await this.updateList()
    }

    find(url: URL) {
        // return get(this._list).find(v => v.url.toString() === url.toString())
    }

    async updateList(){
        const list = await invoke("get_list")
        const map = new Map(Object.entries(list))
        this._list.set(map);
    }

    subscribe(run: Subscriber<Config>) {
		return this._list.subscribe(run)
	}

}

export const list = new InstanceList();