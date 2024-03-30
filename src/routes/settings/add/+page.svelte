<script lang="ts">
    import { list, Protocol, type ServerParams } from '../../../store/server';
    import {goto} from '$app/navigation';
    import { invoke } from '@tauri-apps/api';
    import { error } from 'console';

    const server: ServerParams = {
        protocol: Protocol.https,
        address: 'home.bnjmn.dev',
        port: 443,
        token: ''
    }


    async function addServer() {
        try {
            await invoke("add_instance", {server: {...server}, app: null, })
            goto("/login")
        } catch(err) {
            alert(err) 
        }
    }

    function portSet() {
        if (server.protocol === "http://") {
            server.port = 80
            return
        }
        server.port = 443
    }

</script>
<span class="flex-auto">
<h1 class="h1">Add a server</h1>
<h2>Type in your Home assistant instance URL and Token</h2>
</span>

<div class="input-group input-group-divider grid-cols-[auto_1fr_90px]">
    <select bind:value={server.protocol} on:change={portSet}>
        <option>http://</option>
        <option>https://</option>
      </select>
  <input type="text" placeholder="localhost" bind:value={server.address} />
  <input class="input-group-shim" type="text" placeholder="Input" bind:value={server.port} />
  
</div>

<div>
    <input class="input" type="text" placeholder="Token" bind:value={server.token} />
</div>


<button type="button" class="btn variant-filled" on:click={addServer}>Add Server</button>

