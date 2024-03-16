<script lang="ts">
    import { page } from '$app/stores';
    import { createConnection, getAuth, subscribeEntities } from 'home-assistant-js-websocket';
    
    const hasCode = $page.url.searchParams.has("code");
    
    let code: string;
    let state: string;
    
    if (hasCode) {
        code = $page.url.searchParams.get("code")
        state = $page.url.searchParams.get("state")
        connect()
    } else {
        window.location.href="/login";
    }

    async function connect(){
        const auth = await getAuth()
        const connection = await createConnection({ auth });
        subscribeEntities(connection, (ent) => console.log(ent));
    }
    
    </script>
    
    <h1>{code}</h1>
    <h1>{state}</h1>