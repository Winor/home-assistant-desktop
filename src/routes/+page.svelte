<script lang="ts">
import { page } from '$app/stores';

const hasCode = $page.url.searchParams.has("code");

let code: string;
let state: string;

if (hasCode) {
    code = $page.url.searchParams.get("code")
    state = $page.url.searchParams.get("state")
} else {
    window.location.href="/login";
}

</script>

<h1>{code}</h1>
<h1>{state}</h1>